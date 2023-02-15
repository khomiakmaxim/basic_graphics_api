use std::ops::{Index, IndexMut};

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
            result.x = self.x * inv_length;
            result.y = self.y * inv_length;
            result.z = self.z * inv_length;

        }

        result
    }
}

#[derive(Debug, Clone)]
struct Matrix44<T> {
    m: Vec<Vec<T>>,
}

impl Index<usize> for Matrix44<f64> {
    type Output = Vec<f64>;
    fn index(&self, i: usize) -> &Vec<f64> {
        &self.m[i]
    }
}

impl IndexMut<usize> for Matrix44<f64> {
    fn index_mut(&mut self, i: usize) -> &mut Vec<f64> {
        &mut self.m[i]
    }
}

impl Matrix44<f64> {     
    fn new() -> Matrix44<f64> {
        let mat = Matrix44 {
            m: vec![
                   vec![1., 0., 0., 0.],
                   vec![0., 1., 0., 0.],
                   vec![0., 0., 1., 0.],
                   vec![0., 0., 0., 1.],
                ],
        };

        mat
    }

    fn invert_it (&mut self) -> Matrix44<f64> {
        let mut inv = Matrix44::new();

        // даний підхід теж має проблему, так як в
        // присутності нульового рядка і рядка, який має на 2х
        // позиціях значення, алгоритм цього не замітить і буде ділити на нуль
        for column in 0..4 {
            if self[column][column] == 0. {
                let mut big = column;

                for row in 0..4 {
                    if self[row][column].abs() > self[big][column].abs() {
                        big = row;
                    }
                }

                if big == column {
                    panic!("Singular matrix provided");
                } else {
                    self.m.swap(column, big);
                    inv.m.swap(column, big);                
                }
            }
        }

        // ця частина ітерується по колонках
        // і робить всі значення нульовими, де це є потрібним
        for column in 0..3 { // перебираємо всі колонки, крім останньої
            for row in column+1..4 { // занулюємо всі рядки, нижчі на 1
                let constant = self[row][column] / self[column][column];

                // знаходимо відповідний коефіцієнт для кожного рядка

                // віднімаємо один рядок від іншого
                for j in 0..4 {
                    self[row][j] -= constant * self[column][j];
                    inv[row][j] -= constant * inv[column][j];
                }

                self[row][column] = 0.; // safety measure
            }
        }    


        // backwards substitution
        for row in 0..4 {
            for column in row+1..4 {
                let constant = self[row][column];

                for k in 0..4 {
                    self[row][k] -= self[column][k] * constant;
                    inv[row][k] -= inv[column][k] * constant;
                }

                self[row][column] = 0.;// this saves from round-off error
            }
        }

        // for row in 0..4 {
        //     println!();

        //     for column in 0..4 {
        //         print!("{} ", inv[row][column]);
        //     }
        // }

        // println!();

        // return;
        
        inv
    }

    // this should be implemented via Mul trait
    fn multiply (&self, rhs: &Matrix44<f64>) -> Matrix44<f64> {
        let mut mult = Matrix44 {
            m: vec!(
                vec![1.,0.,0.,0.],
                vec![0.,1.,0.,0.],
                vec![0.,0.,1.,0.],
                vec![0.,0.,0.,1.],
            ),
        };

        for i in 0..4 {
            for j in 0..4 {
                mult[i][j] = self[i][0] * rhs[0][j] +
                             self[i][1] * rhs[1][j] +
                             self[i][2] * rhs[2][j] +
                             self[i][3] * rhs[3][j];
            }
        }

        mult
    }

    fn transpose (&mut self) {
        for i in 0..4 {
            for j in 1..4 {
                let temp = self[i][j];
                self[i][j] = self[j][i];
                self[j][i] = temp;
            }
        }
    }
 
    // [1x4] x [4x4]([3x3] + homogenous coordinates)
    fn multVec (&self, v: &Vec3<f64>) -> Vec3<f64> {
        let mut res = Vec3 { x: 0., y: 0., z: 0. };

        res.x = self[0][0] * v.x + self[1][0] * v.y + self[2][0] * v.z + self[3][0];
        res.y = self[0][1] * v.x + self[1][1] * v.y + self[2][1] * v.z + self[3][1];
        res.z = self[0][2] * v.x + self[1][2] * v.y + self[2][2] * v.z + self[3][2];
        let w = self[0][3] * v.x + self[1][3] * v.y + self[2][3] * v.z + self[3][3];

        if w != 1. && w!= 0. {
            res.x = res.x / w;
            res.y = res.y / w;
            res.z = res.z / w;
        }

        res
    }

    fn scale (&mut self, r: f64) {
        for row in 0..4 {
            for column in 0..4 {
                self[row][column] *= r;
            }
        }
    }
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

    #[test]
    fn cross_product_anticommutative() {
        let vec1 = Vec3::new(0.01,0.12,0.05,);
        let vec2 = Vec3::new(1.,0.,0.,);

        let cross1 = vec1.cross(&vec2);
        let cross2 = vec2.cross(&vec1).scale(-1_f64); 

        assert_eq!(cross1.x, cross2.x);        
        assert_eq!(cross1.y, cross2.y);        
        assert_eq!(cross1.z, cross2.z);

        let vec1 = Vec3::new(0.443,0.34123,123.,);
        let vec2 = Vec3::new(1.,0.,0.,);

        let cross1 = vec1.cross(&vec2);
        let cross2 = vec2.cross(&vec1).scale(-1_f64);

        assert_eq!(cross1.x, cross2.x);
        assert_eq!(cross1.y, cross2.y);        
        assert_eq!(cross1.z, cross2.z);


        let vec1 = Vec3::new(0.01,0.12,0.05,);
        let vec2 = Vec3::new(7.,7.,7.,);

        let cross1 = vec1.cross(&vec2);
        let cross2 = vec2.cross(&vec1).scale(-1_f64);

        assert_eq!(cross1.x, cross2.x);
        assert_eq!(cross1.y, cross2.y);        
        assert_eq!(cross1.z, cross2.z);
    }
}