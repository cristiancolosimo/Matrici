#include<stdio.h>
//metto 100 come massimo dato che con 200 e superiori il programma/compilatore si bugga e spara numeri a caso
#define M_MAX 100
#define N_MAX 100
//creo array già con dimensioni predefinite dato che non so utilizzare i puntatori in c
int main(){
    int A[N_MAX][M_MAX];
    int B[N_MAX][M_MAX];
    int C[N_MAX][M_MAX];
    int n1,m1;//appartenenti a A
    int n2,m2;
    int i,j;//contatori
    int temp;
    n1 = askRigaColonna(1);// input righe
    m1 = askRigaColonna(0);// input Colonne
    //printf("%d %d\n",n1,m1); 
    for(i = 0;i<n1;i++){
        for(j=0;j<m1;j++){
         A[i][j]= inputValore(i,j);   
        }
    }
    n2 = askRigaColonna(1);// input righe
    m2 = askRigaColonna(0);// input Colonne
    for(i = 0;i<n2;i++){
        for(j=0;j<m2;j++){
         B[i][j]= inputValore(i,j);
        }
    }
    if(n1 == n2 && m1 == m2){
        for(i = 0;i<n1;i++){
            for(j = 0;j<m1;j++){
                C[i][j]= A[i][j]+B[i][j];
            }
        }
        printf("Somma eseguita ecco i risultati\n");
        for(i = 0;i<n1;i++){
            for(j = 0;j<m1;j++){
                printf("%d  ",C[i][j]);
                if(j == (m1-1))
                    printf("\n");
            }
        }
    }else {
        printf("Le due matrici hanno dimensione diversa e quindi non possono essere sommate\n");
    }
    return 0;
}
int inputValore(int riga,int col){
    int temp;
    printf("inserisci valore nella riga %d e nella colonna %d\n",riga+1,col+1);
    scanf("%d",&temp);
    return temp;
}
int askRigaColonna(int type){
    int temp;
    if(type==1)
        printf("Quante righe vuoi? max 100\n");
    else
        printf("Quante colonne vuoi? max 100\n");
    
    scanf("%d",&temp);
    if(temp > 100) {
        printf("hai superato il limite\n");
       temp= askRigaColonna(type);
        }
    return temp;
}
