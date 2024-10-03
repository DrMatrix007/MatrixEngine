use matrix_engine::math::matrix::Matrix;

fn main() {
    let m = Matrix::<f32, 4, 4>::one();

    println!("{:10.2}", &m * &m);
}
