#[derive(Debug)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl Vec3<f64> {
    fn new(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3 {
            x,
            y,
            z,
        }   
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn plus(&self, v: &Vec3<f64>) -> Vec3<f64> {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    fn minus(&self, v: &Vec3<f64>) -> Vec3<f64> {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    fn scale(&self, r: f64) -> Vec3<f64> {
        Vec3 {
            x: self.x * r,
            y: self.y * r,
            z: self.z * r,
        }
    }

    fn dot(&self, v: &Vec3<f64>) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    
    fn cross(&self, v: &Vec3<f64>) -> Vec3<f64> {
        Vec3 { 
            x: self.x * v.y - self.y * v.x,
            y: self.z * v.x - self.x * v.z,
            z: self.y * v.z - self.z * v.y,
        }
    }

    fn get_normalized(&self) -> Vec3<f64> {
        let len2 = self.dot(self);  
        let mut result = Vec3::new(0., 0., 0.);

        if len2 > 0. {            
            let inv_length = 1. / len2.sqrt();
            println!("\n\n dot - {len2}  inv_length - {inv_length}, Vec3 - {:?} \n\n", &self);
            result.x = self.x * inv_length;
            result.y = self.y * inv_length;
            result.z = self.z * inv_length;
            println!("\n\n Vec3 - {:?} \n\n", &result);

        }

        result
    }
}

fn main() {
    let my_vec3 = Vec3::new(0.,0.,0.,);
    let mut normalized = my_vec3.get_normalized();
    println!("{:?}", normalized);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_normalized() {
        let my_vec3 = Vec3::new(0.,0.,0.,);
        let mut normalized = my_vec3.get_normalized();
        assert_eq!(normalized.length(), 0_f64);
    }

    #[test]
    fn non_zero_normalized() {
        let vec1 = Vec3::new(0.01,0.12,0.05,);
        let vec2 = Vec3::new(1.,0.,0.,);
        let vec3 = Vec3::new(0.,1.,0.,);
        let vec4 = Vec3::new(0.,0.,1.,);
        let vec5 = Vec3::new(0.4,0.123,123.,);
        let vec6 = Vec3::new(0.443,0.34123,123.,);
        let vec7 = Vec3::new(12.,2.,0.,);
        let vec8 = Vec3::new(1.,5.,1000.,);
        let vec9 = Vec3::new(32.,1000.,0.12,);
        let vec10 = Vec3::new(7.,7.,7.,);

        assert!(f64::abs(vec1.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec2.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec3.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec4.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec5.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec6.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec7.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec8.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec9.get_normalized().length() - 1_f64) < 0.0001);
        assert!(f64::abs(vec10.get_normalized().length() - 1_f64) < 0.0001);
    }
}