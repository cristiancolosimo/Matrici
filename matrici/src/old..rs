type input = i32; //alias di un intero con segno
type inputdim = usize;
//funzioni che utilizzo sempre per fare input
fn inputnumVal() -> input {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Errore nel leggere la linea");
    let out: input = input.trim().parse().unwrap();
    return out;
}

fn inputnum_usize() -> inputdim {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Errore nel leggere la linea");
    let out: inputdim = input.trim().parse().unwrap();
    return out;
}
fn inputstring() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Errore nel leggere la linea");
    let len = input.len();
    input.truncate(len - 2);
    return input;
}
//main

fn main() {
    let mut A: Vec<Vec<input>> = Vec::new();
    let n1: inputdim; //valore senza segno
    let m1: inputdim;
    n1 = ask_riga_colonna(true);
    m1 = ask_riga_colonna(false);
    input_matrice(n1, m1, &mut A);
    //mostra_matrice(n1, m1, &A);
    let mut B: Vec<Vec<input>> = Vec::new();
    let n2: inputdim; //valore senza segno
    let m2: inputdim;
    n2 = ask_riga_colonna(true);
    m2 = ask_riga_colonna(false);
    input_matrice(n2, m2, &mut B);
    let mut C: Vec<Vec<input>> = Vec::new();

    if (n1 == n2 && m1 == m2) {
        println!("Eseguo la somma");
        for i in 0..n1 {
            let mut temp = Vec::new();
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
fn mostra_matrice(riga: inputdim, col: inputdim, matrice: &Vec<Vec<input>>) {
    for i in 0..riga {
        for j in 0..col {
            print!("{}  ", matrice[i][j]);
            if (col - 1) == j {
                println!();
            }
        }
    }
}
fn input_matrice(riga: inputdim, col: inputdim, matrice: &mut Vec<Vec<input>>) {
    for i in 0..riga {
        // il for più bello mai esistito
        let mut temp: Vec<input> = Vec::new();
        for j in 0..col {
            println!("inserisci valore nella riga {} e nella colonna {}", i, j);
            temp.push(inputnumVal());
        }
        matrice.push(temp);
    }
}
fn ask_riga_colonna(typein: bool) -> inputdim {
    if typein {
        println!("Quante righe vuoi?");
    } else {
        println!("Quante colonne vuoi?");
    }
    inputnum_usize()
}
