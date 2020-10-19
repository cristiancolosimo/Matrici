# da eseguire con la versione 3.8.x di python 
import numpy as np # numpy libreria installata con pip install numpy 
def ask_riga_colonna(tipo):
    if tipo:
        print("Quante righe vuoi ?")
    else:
        print("Quante colonne vuoi ?")
    return int(input())

def input_matrice(righe,col):# si l'algoritmo è simile a quello fatto in rust dato che ho fatto prima quello in c poi in rust e poi questo in python
    tempmatrix = []
    for i in range(righe) :# è più bello il for in rust
        temp=[]
        for j in range(col):
            print("Inserisci il valore nella riga {} e colonna {}".format(i,j))
            temp.append(int(input()))
        tempmatrix.append(temp)
    return np.matrix(tempmatrix)
    
def mostra_matrice(matrice):
    print(matrice)             

n1 = ask_riga_colonna(True)
m1 = ask_riga_colonna(False)
A = input_matrice(n1,m1)
    

n2 = ask_riga_colonna(True)
m2 = ask_riga_colonna(False)
B = input_matrice(n2,m2)

if (A.shape == B.shape):# vede direttamente se le dimensioni sono uguali o diverse , alternativa  (n1==n2 and m1 ==m2)
    print("Eseguo la somma")
    C = A+B
    print("Ecco il risultato")
    mostra_matrice(C)
else:
    print("La somma non può essere eseguita dato che le dimensioni delle due matrici sono diverse")