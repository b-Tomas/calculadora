mod math;
mod structs;
mod exp_interpreter;

fn main() {}
pub fn inverse_matrix (m: &Matrix) -> Result<Matrix, Box<dyn Error>>{
	if !m.is_squared(){
		return Err("Bad dimensions");
	} else {
		let aux: f32 = det(&m).unwrap();
		if aux != 0{
			let trasp : Matrix = trasp_squared_matrix(&m).unwrap();
			let adj : Matrix = adj_calculus(&trasp).unwrap(); //Calculo el adjunto de la traspuesta
			let	inverse : Matrix = inverse_calculus(&adj,aux);
		}	
	}