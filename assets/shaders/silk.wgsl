/// by TrueBoolean: https://www.shadertoy.com/view/Xs3SRn

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BackgroundMaterial {
    time: f32,
};

@group(2) @binding(0)
var<uniform> background: BackgroundMaterial;

fn height(p: vec2<f32>) -> f32 {
	return sin(p.x) + sin(p.x + p.y) + cos(p.y) / 1.5 + sin(background.time + p.x) + 5.;
}

fn map(p: vec3<f32>) -> f32 {
	return p.y - height(p.xz);
}

fn rotate(x: f32) -> mat2x2<f32> {
    return mat2x2(cos(x), -sin(x), sin(x), cos(x));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
	let uv = mesh.uv;
//	let uv = (mesh*2.-iResolution.xy)/min(iResolution.x, iResolution.y);

    let ray = normalize(vec3(uv,1.));
    let yz = ray.yz * rotate((sin(background.time)/3.+1.5));
    let xz = ray.xz * rotate((sin(background.time)/2.+1.)/5.);
    let newRay = vec3(xz.x, yz.x, xz.y * yz.y);

    var t = 0.;
    for (var i = 1; i <= 29 ; i = i + 1) {
        t += map(vec3(background.time,0.,background.time/2.)+newRay*t)*.5;
    }

    let fog = 1./(1.+t*t*.005);
    let fc = vec3(fog*fog, fog/2., fog);
	return vec4(fc, 1.);
}

