# Language Rust
# Introduction
Rust est un nouveau langage de programmation open source créé par Mozilla et une communauté de volontaires, conçu pour aider les développeurs à concevoir des applications ultra-rapides et sécurisées qui utilisent avantageusement les puissantes fonctionnalités offertes par les processeurs multi-cœurs modernes. Il permet d'éviter les erreurs de segmentation et assure la sécurité des threads, le tout avec une syntaxe facile à assimiler.
# Bases du language
## Premier programme
Pour créer un premier programme, nous allons créer un fichier main.rs (rs est l'extention des fichiers Rust) qui va contenir le code: 

```markdown
fn main() {
    println!("Bonjour tous le monde!");
}
Ce petit programme imprime le texte "Bonjour tous le monde" à l'écran.
```
## Les variables
En Rust, toutes les variables locales sont déclarées par let et sont des constantes (on ne peut pas changer sa valeur). Il faut ajouter le déclarateur mut, pour pouvoir les déclarer comme variable mutable.

### Exemple variable constante
```markdown
fn main() {
    let x = 0;
    x = 4; // Affiche un message d'erreur ==> error[E0384]: cannot assign twice to immutable variable `x`
    println!("La valeur de x est : {}", x);
}
```

### Exemple variable mutable
```markdown
fn main() {
    let mut x = 0;
    x = 4; // ok
    println!("La valeur de x est : {}", x);// affiche La valeur de x est : 4
}
```
## Les types des variables
Chaque valeur dans Rust est associée à un type de données comme les autres languages de programmation. Donc on trouve: des entiers, des booleans, des flottants, des chaines de caracères(Strings)...Mais la déclaration d'une variable avec un type est différentes en Rust par rapport aux autres languages.

### Les entiers
Un nombre entier est un nombre que l'on peut écrire sans virgule. En Rust les types d'entiers signés commencent par i et non signés par u.
Exemple des entiers:
 ```markdown
 
 let i : i8 = 0; // Entier signé qui occupe 8 bits d'espace
 let i : i32 = 0; //Entier signé qui occupe 32 bits d'espace
let i = 7i18;//Entier signé qui occupe 18 bits d'espace

 ```
### Les booleans
un type booléen dans Rust a deux valeurs possibles: true et false. Le type booléen dans Rust est spécifié en utilisant bool. 
exemple:

 ```markdown
 
fn main() {
    let a: bool = true; // type explicite en utilisant bool
    let b = false; // le type est boolean implicitement avec la valeur false
}
 ```
### Les flottants
Un flottant est un nombre avec la virgule. Les types à virgule flottante de Rust sont f32 et f64, qui sont respectivement de 32 bits et 64 bits. Le type par défaut est f64.

 ```markdown
fn main() {

    let x = 0.0; // Variable de type float 64 bit: type inplicite
    let xx: f64 = 0.0; // Variable de type float 64 bit: type explicite 
}
 ```
  
### Char 
Le type de caractère de Rust est le type alphabétique le plus primitif de la langue, et le code suivant montre une façon de l'utiliser:

```markdown
fn main() {
    let a = 'a';// caractère 'a'
    let b = 'B';
}
```
## Types composés

Les types composés peuvent regrouper plusieurs valeurs en un seul type

### Tuple
Un tuple est un moyen général de regrouper en un seul type un certain nombre de valeurs avec une des types différents en un seul type composé. Les tuples ont une longueur fixe.

Nous créons un tuple en écrivant une liste de valeurs séparées par des virgules entre parenthèses. Chaque position dans le tuple a un type, et les types des différentes valeurs dans le tuple n'ont pas à être les mêmes.
Exemple:
```markdown

let t = (0, 5.0, false, "Rust");

```

### Array

Les tableaux sont une suite de valeurs, chaque élément d'un tableau doit avoir le même type. les tableaux dans Rust ont une longueur fixe.

```markdown

let nombres = [1, 2, 3, 4, 5];

```
## Fonctions
Les fonction dans Rust commencent par fn et elles ont un ensemble de parenthèses après le nom de la fonction. Les accolades indiquent au compilateur où commence et se termine le corps de la fonction.

```markdown

fn main() {
    println!("Utilsation des fonctions");

    ma_fonction();// Appel de la conction
}

fn ma_fonction() {

    println!("Text à partir de la fonction");
}
```

Les fonctions peuvent également être définies pour avoir des paramètres. Lorsqu'une fonction a des paramètres, nous pouvons lui fournir des valeurs concrètes pour ces paramètres. Les fonctions peuvent aussi renvoyer des valeurs. Nous ne nommons pas les valeurs de retour, mais nous déclarons leur type après une flèche (->) :

```markdown 

fn nombre() -> i32 {
    1
}

fn main() {
    let num = nombre();

    println!("num : {}", num);
}


```
## Condtions

### if / else if / else

Les if / else if / else fonctionnent de la même façon qu'en C/C++/Java :
```markdown 

let number: i32 = 99;

if number >= 100 {
    println!("Supérieur à 100 !");
} else {
    println!("Inférieur à 100 !");
}
### match
match correspand à le commutateur switch aux autres languages comme en C.

```markdown 

let boole = false;

match boole {
    true => {
        println!("Vrai");
    }
    false => {
        println!("Faux");
        // traitement
    }
}

```
## Les boucles 
### Loop

La boucle est une boucle indéfinie (toujours vrai) ou une boucle infinie. Elle s'arrete avec le mot-clé break . 
```markdown 
let mut i: i32 = 0;

loop {
    println!(" Language Rust ");
    i += 1;
    if i > 3 {
        break;
    }
}

```
### For

peut être utilisé pour itérer sur un Iterator. L'une des façons les plus simples de créer un itérateur est d'utiliser la notation de l'intervale a..b. Cela donne des valeurs de a (inclus) à b (exclus) par pas de 1.

Exemple:

```markdown 
for i in 0..10 {
    if n % 2 == 0 {
            println!("Nombre {} est paire",i);
        } else if n % 2 == 1 {
            println!("Nombre {} est impaire",i);
        }   
}
```
### While
 La boucle while exécute les instructions chaque fois que la condition spécifiée est évaluée à vrai.
 
 ```markdown 
let mut i: i32 = 0;
while(i<3) {
            println!("Language Rust ");
            i += 1;
}

## Propiétaire (Ownership)

La propriétaire est une caractéristique centrale de Rust et parmit l'un de ses points forts. Un système de propriété gère la mémoire avec un ensemble de règles que le compilateur vérifie au moment de la compilation.

En Rust chaque valeur a une variable nommée propriétaire (Owner) de la valeur. Chaque donnée stocké dans Rust sera associé à un propriétaire. Il faut savoir que :

- Chaque donnée ne peut avoir qu'un seul propriétaire à la fois.

- Deux variables ne peuvent pas pointer vers le même emplacement mémoire. 

Exemple:
```markdown 
fn main() 
{ 
    let matier_p : String = String::from("AWS"); 
    let matier_d : String = matier_p ; // affecter matier_p à matier_d
    let matier_t : String = matier_p; // Erreur!! parce que  la ressource a été déplacée.
}

```

Dans le cas des types primitifs, le contenu d'une variable est copié dans une autre. Il n'y a donc pas de transfert de propriété. En effet, une variable primitive a besoin de moins de ressources qu'un objet. 

## Slice

Slice est un pointeur sur un bloc de mémoire. Les slices peuvent être utilisées pour accéder à des parties de données stockées dans des blocs de mémoire contigus. Elle peut être utilisée avec des structures de données comme des tableaux, des vecteurs et des strings. 
Les slices sont des pointeurs vers les données réelles.La taille d'une slice est déterminée lors de l'exécution.

Par exemple, des slices peuvent être utilisées pour récupérer une partie d'une valeur d'une chaîne de caractères. Par conséquent, nous devons spécifier l'indice de début et de fin d'une chaîne.
```markdown 
let array: [i32;4] = [5, 8, 1, 4];

let tranche = &array[1..2]; // tranche contient [8,1]

```


