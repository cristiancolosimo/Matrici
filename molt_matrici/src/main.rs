//funzione che fa il wrapping per stdin()
fn input_num<T: std::str::FromStr>() -> T {
    //tramite i generict T posso scrivere una funzione una volta e riutilizzarla per più tipi, in fase di compilazione per ogni tipo usato ne verra creata una
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Errore nel leggere la linea");
    let out: T = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Valore non valido, rinserisci il valore");
            input_num()
        }
    };
    return out;
}
fn ask_riga_colonna(typein: bool) -> usize {
    if typein {
        println!("Quante righe vuoi?");
    } else {
        println!("Quante colonne vuoi?");
    }
    input_num()
}

struct Matrice {
    // le struct sono simili agli oggetti
    righe: usize,
    colonne: usize,
    matrix: Vec<Vec<i32>>,
}
impl Matrice {
    fn new(righe: usize, colonne: usize) -> Matrice {
        let matrice: Vec<Vec<i32>> = Vec::new();
        Matrice {
            righe,
            colonne,
            matrix: matrice,
        }
    }
    fn set_matrice(&mut self, matrice: Vec<Vec<i32>>) {
        self.matrix = matrice;
    }
    fn input_matrice(&mut self) {
        for i in 0..self.righe {
            let mut temp: Vec<i32> = Vec::new();
            for j in 0..self.colonne {
                println!("inserisci valore nella riga {} e nella colonna {}", i, j);
                temp.push(input_num::<i32>());
            }
            self.matrix.push(temp);
        }
    }
    fn mostra_matrice(&self) {
        for i in 0..self.righe {
            for j in 0..self.colonne {
                print!("{}  ", self.matrix[i][j]);
                if (self.colonne - 1) == j {
                    println!();
                }
            }
        }
    }
    fn somma(A: Matrice, B: Matrice) -> Result<Matrice, String> {
        if A.colonne == B.colonne && A.righe == B.righe {
            let mut C: Matrice = Matrice::new(A.righe, A.colonne);
            for i in 0..A.righe {
                let mut temp: Vec<i32> = Vec::new();
                for j in 0..A.colonne {
                    let A = A.matrix[i][j]; //shadowing, viene ridichiarata la variabile, con tipo diverso, ma ha effetto solo nel blocco corrente
                    let B = B.matrix[i][j];
                    temp.push(A + B);
                }
                C.matrix.push(temp);
            }
            Ok(C)
        } else {
            Err(String::from(
                "Somma non valida, le dimensioni delle due matrici sono diverse",
            ))
        }
    }
    fn moltiplicazione(a: Matrice, b: Matrice) -> Result<Matrice, String> {
        if a.colonne != b.righe {
            return Err(String::from("Moltiplicazione non effettuabile"));
        }
        let mut C = Matrice::new(a.righe, b.colonne);
        for i in 0..C.righe {
            let mut temp: Vec<i32> = Vec::new();
            for j in 0..C.colonne {
                let mut tempcalc = 0;
                for m in 0..b.righe {
                    tempcalc += a.matrix[i][m] * b.matrix[m][j];
                }
                temp.push(tempcalc);
            }
            C.matrix.push(temp);
        }
        Ok(C)
    }
}
//main

fn main() {
    //let mut A = Matrice::new(ask_riga_colonna(true), ask_riga_colonna(false));
    //A.input_matrice();
    let mut A = Matrice::new(1, 3);
    A.set_matrice(vec![vec![-2, 1, -3]]);
    //let mut B = Matrice::new(ask_riga_colonna(true), ask_riga_colonna(false));
    //B.input_matrice();
    let mut B = Matrice::new(3, 1);
    B.set_matrice(vec![vec![1], vec![2], vec![3]]);
    //A.mostra_matrice();
    let C = Matrice::moltiplicazione(A, B).unwrap(); //uso unwrap così se c'è un errore crasha il programma, se no restituisce il risultato senza utilizzare il match
    C.mostra_matrice();
    //match Matrice::somma(A, B) {//il match è uno switch ma molto più figo e potente
    //    Ok(v) => v.mostra_matrice(),
    //    Err(v) => println!("{}", v),
    //}
}
