type Input = i32; //alias di un intero con segno
type Inputdim = usize; //alias di un usize , gli usize vengono usati per

//funzione che fa il wrapping per stdin()
fn input_num<T: std::str::FromStr>() -> T {
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
//main

fn main() {
    let mut A: Vec<Vec<Input>> = Vec::new(); //in rust le variabili che stanno nel heap(quindi i puntatori) vengono eliminani quando non vengono più usati in automatico
    let n1: Inputdim; //valore senza segno
    let m1: Inputdim;
    n1 = ask_riga_colonna(true);
    m1 = ask_riga_colonna(false);
    input_matrice(n1, m1, &mut A);
    //mostra_matrice(n1, m1, &A);
    let mut B: Vec<Vec<Input>> = Vec::new();
    let n2: Inputdim; //valore senza segno
    let m2: Inputdim;
    n2 = ask_riga_colonna(true);
    m2 = ask_riga_colonna(false);
    input_matrice(n2, m2, &mut B);
    let mut C: Vec<Vec<Input>> = Vec::new();
    if (n1 == n2 && m1 == m2) {
        //parentesi inutili, ma le ho messe solo per far vedere che rust è molto flessibile
        println!("Eseguo la somma");
        for i in 0..n1 {
            let mut temp = Vec::new(); //vettore temporaneo
            for j in 0..m1 {
                temp.push(A[i][j] + B[i][j]);
            }
            C.push(temp);
        }
        println!("Risultato somma:");
        mostra_matrice(n1, m1, &C);
    } else {
        println!("Somma non valida, le dimensioni delle due matrici sono diverse");
    }
}
fn mostra_matrice(riga: Inputdim, col: Inputdim, matrice: &Vec<Vec<Input>>) {
    for i in 0..riga {
        // il for più bello mai esistito
        for j in 0..col {
            print!("{}  ", matrice[i][j]);
            if (col - 1) == j {
                println!();
            }
        }
    }
}
fn input_matrice(riga: Inputdim, col: Inputdim, matrice: &mut Vec<Vec<Input>>) {
    for i in 0..riga {
        let mut temp: Vec<Input> = Vec::new();
        for j in 0..col {
            println!("inserisci valore nella riga {} e nella colonna {}", i, j);
            temp.push(input_num::<i32>());
        }
        matrice.push(temp);
    }
}
fn ask_riga_colonna(typein: bool) -> Inputdim {
    if typein {
        println!("Quante righe vuoi?");
    } else {
        println!("Quante colonne vuoi?");
    }
    input_num::<usize>()
}
