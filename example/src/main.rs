mod helpers;
use black::{ Raster, DepthBuffer, FragmentProgram, Interpolate, VertexProgram };
use black::{ Mat4, Vec2, Vec3, Vec4 };
use helpers:: { Builder,  Geometry, Vertex, Sampler };

pub struct Uniform {
    pub projection: Mat4,
    pub view:       Mat4,
    pub matrix:     Mat4,
    pub sampler:    Sampler,
    pub light:      Vec3
}

#[derive(Interpolate)]
struct Varying {
    pub position: Vec4,
    pub normal:   Vec3,
    pub uv:       Vec2,
}

struct VertexShader; impl VertexProgram for VertexShader {
    type Uniform = Uniform;
    type Vertex  = Vertex;
    type Varying = Varying;
    fn main(&self, uniform: &Uniform, input: &Vertex, varying: &mut Varying) -> Vec4 {
        varying.position  = input.position;
        varying.normal    = input.normal;
        varying.uv        = input.uv;
        input.position * 
            (uniform.matrix * 
                (uniform.view * uniform.projection))
    }
}

struct FragmentShader; impl FragmentProgram for FragmentShader {
    type Uniform = Uniform;
    type Varying = Varying;
    fn main(&self, uniform: &Uniform, varying: &Varying) -> Vec4 {
        let e2p   = Vec3::normalize(&varying.position.xyz()) * -1.0;
        let l2p   = Vec3::normalize(&(varying.position.xyz() - uniform.light));
        let ldp   = Vec3::dot(&varying.normal, &l2p);
        
        // very rough specular
        let specular_term  = Vec3::all(1.0) *  Vec3::new(1.0, 1.0, 1.0);
        let reflect_term   = Vec3::dot(&Vec3::reflect(&l2p, &varying.normal), &e2p);
        let specular       = (specular_term * reflect_term).xyzw() * ldp;
        let color          = uniform.sampler.get(varying.uv.x, varying.uv.y);
        
        if ldp < 0.0 {
            Vec4::new(0.0, 0.0, 0.0, 1.0)
        } else {
            color * specular // * Vec4::new(varying.normal.x, varying.normal.y, varying.normal.z, 1.0)
        }
    }
}

fn main() {
    
    let pixel_size  = 1;
    let width       = (120 * 8) / pixel_size;
    let height      = (60  * 8) / pixel_size;
    let mut time    = 0.0;

    // window context
    let mut context = Builder::default()
        .pixel_size(pixel_size)
        .size(width, height)
        .position(2000, 10)
        .create()
        .unwrap();

    let mut depth   = DepthBuffer::new(width, height);
    let mut uniform = Uniform {
        sampler:    Sampler::new(8.0),
        light:      Vec3::new(0.0, -10.0, 0.0),
        projection: Mat4::perspective_fov(70.0 * 3.14 / 180.0, width as f32 / height as f32, 0.1, 1000.0),
        matrix:     Mat4::identity(),
        view:       Mat4::look_at(
            &Vec3::new(0.0, 2.5, 3.0),
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        ),
    };
    
    let geometry = Geometry::obj("./models/bunny.obj").unwrap();
    // let geometry = Geometry::obj("./models/teapot.obj").unwrap();
    // let geometry = Geometry::cube(1.0);
    while context.active() {
        // clear buffers
        context.clear(0x000000);
        depth.clear();
        
        // update uniforms
        uniform.matrix = Mat4::rotation_y(time);// * Mat4::rotation_x(time);
        uniform.light.x = f32::cos(time * 4.2) * 10.0;
        uniform.light.z = f32::sin(time * 4.2) * 10.0;
        uniform.view =  Mat4::look_at(
            &Vec3::new(f32::sin(time * 0.5) * 2.0, 2.25, f32::cos(time * 0.5) * 3.0),
            &Vec3::new(0.0, -0.1, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        );
        
        // render triangles
        for _ in 0..1 {
        for n in (0..geometry.indices.len()).step_by(3) {
            let i0 = geometry.indices[n + 0];
            let i1 = geometry.indices[n + 1];
            let i2 = geometry.indices[n + 2];
            let v0 = &geometry.vertices[i0];
            let v1 = &geometry.vertices[i1];
            let v2 = &geometry.vertices[i2];
            Raster::triangle(
                &VertexShader,
                &FragmentShader,
                &mut depth,
                &mut context,
                &uniform,
                v0,
                v1,
                v2,
            );
        }
        }
        context.present().unwrap();
        time = time + 0.01;
    }
}