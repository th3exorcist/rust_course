pub fn is_divisible_by(number: u32, divisor: u32) -> u32 {
	return number % divisor == 0;
}

pub fn convert(number: u32) -> String {

	return number.to_string();
}
	
/* baguncei tudo, igual fosse Python */
pub fn raindrops(n: u32) -> String {
    
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let factors = [3,5,7];
    let mut total = String::new(); // string

    for factor in sounds.iter() {

    	if is_divisible_by(sounds, factors) {
    		
    		total.push_str(factor.1);
    	}
    }

    if total.len() > 0 {

    	return total;
    }

    return convert(n); // string
    
    }

