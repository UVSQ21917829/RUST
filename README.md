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
```
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
```
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
## Vecteurs
Un «vecteur» est un tableau dynamique ou «évolutif», implémenté comme le type de bibliothèque standard Vec <T>. Le T signifie que nous pouvons avoir des vecteurs de tout type. Les vecteurs allouent toujours leurs données sur le tas. 
Les vecteurs stockent leur contenu sous forme de tableaux contigus de T sur le tas. Cela signifie qu'ils doivent être capables de connaître la taille de T au moment de la compilation . La taille de certaines choses ne peut pas être connue au moment de la compilation.
Exemple :

```markdown 
let v = vec![1, 2, 3, 4, 5];

```  
## Enums
Une énumération dans Rust est un type qui représente des données qui sont l'une des nombreuses variantes possibles. Chaque variante de l'énumération peut éventuellement avoir des données associées:
exemple :
```markdown 
enum Location {
    Unknown,
    Anonymous,
    Known(Coord),
}


```
## Strucs
Les structures sont un moyen de créer des types de données plus complexes. 
Exemple :
```markdown 
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0 }; // origin: Point

    println!("l'origine est ({}, {})", origin.x, origin.y);
}

```
# Bibliotheques
## Crates 
Les caisses sont un peu similaires aux packages dans certaines autres langues. Les caisses se compilent individuellement. Si la caisse a des modules de fichiers enfants, ces fichiers seront fusionnés avec le fichier de caisse et compilés en une seule unité.

 Une caisse peut produire un exécutable / un binaire ou une bibliothèque. src / main.rs est la racine de la caisse / le point d'entrée pour une caisse binaire et src / lib.rs est le point d'entrée pour une caisse de bibliothèque.
 Ici nous utilisons des exemples les plus simples de quelques bibliothèque tierces :
## Rand
Une bibliothèque Rust pour la génération de nombres aléatoires.
Rand fournit des utilitaires pour générer des nombres aléatoires, pour les convertir en types et distributions utiles, et certains algorithmes liés à l'aléatoire.
Les principaux traits de génération de nombres aléatoires de Rand vivent dans la caisse rand_core , Les implémentations RNG devraient préférer utiliser rand_core tandis que la plupart des autres utilisateurs devraient dépendre de rand.
## bitflags
Un générateur d'indicateurs de masque de bits de typeafe utile pour les ensembles d'indicateurs de masques de bits de style C. Il peut être utilisé pour créer des wrappers de typeafe autour des API C.
Les bitflags! macro génère une structure qui gère un ensemble d'indicateurs. Les indicateurs ne doivent être définis que pour les types entiers, sinon des erreurs de type inattendues peuvent se produire au moment de la compilation.

## rand_core
Traits de base et types d'erreur de la bibliothèque rand, ainsi que des outils pour implémenter les RNG.

Cette caisse est destinée à être utilisée lors de la mise en œuvre du trait principal, RngCore; il définit les traits de base à implémenter ainsi que plusieurs petites fonctions pour faciliter leur implémentation et les types requis pour la gestion des erreurs.

La caisse principale rand réexporte la plupart des éléments définis dans cette caisse, ainsi que des outils pour convertir les échantillons entiers générés par RngCore dans de nombreuses applications différentes (y compris l'échantillonnage à partir de plages restreintes, la conversion en virgule flottante, les permutations de liste et l'initialisation sécurisée des RNG). La plupart des utilisateurs devraient préférer utiliser la caisse principale du rand.

## unicode_xid
Déterminez si un caractère est un identifiant valide pour un analyseur et / ou un lexeur conformément aux règles de l'Annexe 31 de la norme Unicode.
Exemple d'utilisation :
```markdown 
extern crate unicode_xid;

use unicode_xid::UnicodeXID;

fn main() {
    let ch = 'a';
    println!("Is {} a valid start of an identifier? {}", ch, UnicodeXID::is_xid_start(ch));
}

```
## Serde
Serde est un framework  pour la sérialisation et la désérialisation des structures de données Rust de manière efficace et générique.
## Rust Quasi-Quoting
Cette caisse fournit la citation! macro pour transformer les structures de données de l'arbre de syntaxe Rust en jetons de code source.

Les macros procédurales dans Rust reçoivent un flux de jetons en entrée, exécutent du code Rust arbitraire pour déterminer comment manipuler ces jetons et produisent un flux de jetons à remettre au compilateur pour le compiler dans la caisse de l'appelant. La quasi-citation est une solution à une partie de cela - la production de jetons à retourner au compilateur.

Cette caisse est motivée par le cas d'utilisation des macros procédurales, mais est une bibliothèque à usage général de quasi-cotation Rust et n'est pas spécifique aux macros procédurales.

## log
La caisse à journaux fournit une API de journalisation unique qui résume la mise en œuvre réelle de la journalisation. Les bibliothèques peuvent utiliser l'API de journalisation fournie par cette caisse, et le consommateur de ces bibliothèques peut choisir l'implémentation de journalisation qui convient le mieux à son cas d'utilisation.

Si aucune implémentation de journalisation n'est sélectionnée, la façade revient à une implémentation "noop" qui ignore tous les messages de journal. La surcharge dans ce cas est très petite - juste une charge entière, une comparaison et un saut.

Une demande de journal se compose d'une cible, d'un niveau et d'un corps. Une cible est une chaîne qui prend par défaut le chemin du module de l'emplacement de la demande de journal, bien que cette valeur par défaut puisse être remplacée. Les implémentations de l'enregistreur utilisent généralement la cible pour filtrer les demandes en fonction d'une configuration utilisateur.
## lazy-static.rs
Une macro pour déclarer la statique évaluée paresseusement dans Rust.

En utilisant cette macro, il est possible d'avoir des statiques qui nécessitent que le code soit exécuté au moment de l'exécution pour être initialisé. Cela inclut tout ce qui nécessite des allocations de tas, comme les vecteurs ou les mappages de hachage, ainsi que tout ce qui nécessite le calcul d'appels de fonction non const.
