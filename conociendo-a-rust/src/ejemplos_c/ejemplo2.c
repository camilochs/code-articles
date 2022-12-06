#include <stdio.h>
#include <string.h>

struct Libros {
   char  titulo[50];
   char  autor[50];
   int   identificador;
};

void mostrarLibro(struct Libros libro) {

   printf("Libro título: %s \n", libro.titulo);
   printf("Libro autor: %s \n", libro.autor);
   printf("Libro id: %d \n", libro.identificador);
}

void modificarTitulo(struct Libros* libro, char titulo[50]){
    strcpy(libro->titulo, titulo);
}

int main(int argc, char const *argv[])
{
    struct Libros PrimerLibro;
    struct Libros SegundoLibro;

    strcpy(PrimerLibro.titulo, "Don Quijote de la Mancha");
    strcpy(PrimerLibro.autor, "Miguel de Cervantes"); 
    PrimerLibro.identificador = 1;

    modificarTitulo(&PrimerLibro, "Don Quijote de la Mancha - Edición conmemorativa");
    mostrarLibro(PrimerLibro);

    return 0;
}
