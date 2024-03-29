# Instalação

## Linux ou macOS,

$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

Após a instalação para verificar a versão instalada digite:

rustc --version

## Comando para atualizar o rust

rustup update

## Desinstalar o rust

rustup self uninstall

# Escrevendo um programa

Os arquivos Rust sempre terminam com a extensão .rs

Nome do arquivo: main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

### Executando

```shell
$ rustc main.rs
$ ./main
Hello, world!
```

# Cargo
Cargo é o sistema de construção e gerenciador de pacotes do Rust. 

```shell
$ cargo --version
```

## Criando um Projeto
```shell
$ cargo new hello_cargo
```
No Rust, os pacotes de código são chamados de crates.
crates.io
 
## Construindo 

Este comando gera o build do projeto para ser executado posteriormente.

```shell
$ cargo build
```

## Executando

Para compilar e executar o projeto podemos utilizar o seguinte comando:
```shell
$ cargo run
```

## Checando
Este comando verifica rapidamente seu código para garantir que ele seja compilado.
Ele não gera o binário.

```shell
$ cargo check
```

## Release
Para compilá-lo com otimizações. Este comando criará um executável em target/release em vez de target/debug. As otimizações fazem com que seu código Rust seja executado mais rápido, mas ativá-las aumenta o tempo de compilação do programa.
```shell
$ cargo build --release
```

## Exemplo

```toml
[dependencies]
rand = "0.8.5"
```


```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

# Palavras reservadas

Lembre-se de que você não pode usar essas palavras como nomes de variáveis ​​ou funções.

Palavras-chave atualmente em uso
A seguir está uma lista de palavras-chave atualmente em uso, com suas funcionalidades descritas.

* as- realizar conversão primitiva, eliminar a ambiguidade da característica específica que contém um item ou renomear itens em use instruções
* async- retornar a Future em vez de bloquear o thread atual
* await- suspender a execução até que o resultado de a Future esteja pronto
* break- sair de um loop imediatamente
* const- definir itens constantes ou ponteiros brutos constantes
* continue- continue para a próxima iteração do loop
* crate- em um caminho de módulo, refere-se à raiz da caixa
* dyn- envio dinâmico para um objeto de característica
* else- substituto ife if letcontrole de construções de fluxo
* enum- definir uma enumeração
* extern- vincular uma função ou variável externa
* false- Literal falso booleano
* fn- definir uma função ou o tipo de ponteiro de função
* for- percorrer itens de um iterador, implementar uma característica ou especificar um tempo de vida com classificação mais alta
* if- ramificação baseada no resultado de uma expressão condicional
* impl- implementar funcionalidade inerente ou característica
* in- parte da for sintaxe do loop
* let- vincular uma variável
* loop- loop incondicionalmente
* match- combinar um valor com padrões
* mod- definir um módulo
* move- fazer com que um fechamento se aproprie de todas as suas capturas
* mut- denota mutabilidade em referências, ponteiros brutos ou ligações de padrões
* pub- denota visibilidade pública em campos struct, implblocos ou módulos
* ref- vincular por referência
* return- retorno da função
* Self- um alias de tipo para o tipo que estamos definindo ou implementando 
* self- assunto do método ou módulo atual
* static- variável global ou tempo de vida que dura toda a execução do programa
* struct- definir uma estrutura
* super- módulo pai do módulo atual
* trait- definir uma característica
* true- Booleano verdadeiro literal
* type- definir um alias de tipo ou tipo associado
* union- definir um sindicato ; é apenas uma palavra-chave quando usada em uma declaração de união
* unsafe- denota código, funções, características ou implementações inseguras
* use- trazer símbolos para o escopo
* where- denotam cláusulas que restringem um tipo
* while- faz um loop condicionalmente com base no resultado de uma expressão


Palavras-chave reservadas para uso futuro

As palavras-chave a seguir ainda não possuem nenhuma funcionalidade, mas são reservadas pelo Rust para potencial uso futuro.

* abstract
* become
* box
* do
* final
* macro
* override
* priv
* try
* typeof
* unsized
* virtual
* yield

# Variáveis e Mutabilidade
Por padrão, as variáveis são imutáveis. Quando uma variável é imutável, uma vez que um valor está vinculado a um nome, você não pode alterar esse valor.
Embora as variáveis sejam imutáveis por padrão, você pode torná-las mutáveis adicionando- mut antes do nome da variável.

Em Rust, você pode usar o operador sizeof para determinar o tamanho de uma variável em bytes. Por exemplo, se você tiver
uma variável x, você pode usar std::mem::size_of_val(&x) para obter o tamanho em bytes de x.
```rust
fn main() {
    let x: i32 = 10;
    let size = std::mem::size_of_val(&x);
    println!("O tamanho da variável x é de {} bytes.", size);
}
```

## Constantes
Assim como as variáveis imutáveis, as constantes são valores vinculados a um nome e não podem ser alterados, mas existem algumas diferenças entre constantes e variáveis.

Primeiro, você não tem permissão para usar mut const. As constantes não são apenas imutáveis por padrão – elas são sempre imutáveis. Você declara constantes usando a constpalavra-chave em vez da letpalavra-chave, e o tipo do valor deve ser anotado.

As constantes podem ser declaradas em qualquer escopo, incluindo o escopo global, o que as torna úteis para valores que muitas partes do código precisam conhecer.

A última diferença é que as constantes podem ser definidas apenas como uma expressão constante, não como o resultado de um valor que só poderia ser calculado em tempo de execução.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

 A convenção de nomenclatura do Rust para constantes é usar todas as letras maiúsculas com sublinhados entre as palavras.
 
* não é permitido definir constantes recursivamente.
* tupla .
* array .
* struct.
* Expressões de bloco.
* range
* expressões booleans 
* loop , while e while let expressões.
* if e if let e match expressões .

## Shadowing 

A primeira variável é sombreada (shadowed) pela segunda, o que significa que a segunda variável é o que o compilador verá quando você usar o nome da variável. Na verdade, a segunda variável ofusca a primeira, assumindo qualquer uso do nome da variável para si mesma até que ela mesma fique sombreada(shadowed) ou o escopo termine. 

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```
Este programa primeiro se liga x a um valor de 5. Em seguida, ele cria uma nova variável x repetindo let x =, pegando o valor original e somando 1 então o valor de xé então 6. Então, dentro de um escopo interno criado com chaves, a terceira let instrução também sombreia x e cria uma nova variável, multiplicando o valor anterior por 2 para fornecer x um valor de 12. Quando esse escopo termina, o sombreamento interno termina e x volta a ser 6.

A outra diferença entre mute shadowing é que, como estamos efetivamente criando uma nova variável quando usamos a let palavra-chave novamente, podemos alterar o tipo do valor, mas reutilizar o mesmo nome. 

```rust
    let spaces = "   ";
    let spaces = spaces.len();
```

A primeira spacesvariável é um tipo string e a segunda spacesvariável é um tipo numérico. Assim, o sombreamento nos poupa de ter que inventar nomes diferentes, como spaces_stre spaces_num; em vez disso, podemos reutilizar o spacesnome mais simples. No entanto, se tentarmos usar mutpara isso, como mostrado aqui, obteremos um erro em tempo de compilação:

```rust
    let mut spaces = "   ";
    spaces = spaces.len();
```
O erro diz que não podemos alterar o tipo de uma variável.

#Tipos de dados
Há dois subconjuntos de tipos de dados: escalar e composto
Rust é uma linguagem de tipagem estática, o que significa que ela deve conhecer os tipos de todas as variáveis ​​em tempo de compilação. O compilador geralmente pode inferir que tipo queremos usar com base no valor e como o usamos.

## Tipos escalares
Um tipo escalar representa um único valor. Rust tem quatro tipos escalares principais: inteiros, números de ponto flutuante, booleanos e caracteres. 

### Inteiros
Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

o default é 32 bits. i32

signed
   n - 1      n - 1
-(2      ) a 2       - 1  <=> i8 =  -128 a 127

unsigned
 n     
2  - 1 <=> u8 = 0 a 255

o isize e usize varia de acordo com a sua arquitetura, podendo ser de tamanho 32 bits ou 64 bits.
Ele depende da sua arquitetura. A principal situação em que você usaria isize ou usize é ao indexar algum tipo de coleção.
Observe que os literais numéricos que podem ser vários tipos numéricos permitem um sufixo de tipo, como 57u8, para designar o tipo.
Literais numéricos também podem ser usados _​​como separadores visuais para facilitar a leitura do número, como 1_000, que terá o mesmo valor como se você tivesse especificado 1000.

Quando você está compilando no modo de depuração, Rust inclui verificações de estouro de número inteiro que fazem com que seu programa entre em panic error em tempo de execução se esse comportamento ocorrer.

Quando você está compilando no modo de liberação com o --release, Rust não inclui verificações de estouro de número inteiro que causa panic error. Em vez disso, se ocorrer overflow, no caso de a u8, o valor 256 torna-se 0, o valor 257 torna-se 1 e assim por diante. O programa não entrará em panic erros, mas a variável terá um valor que provavelmente não é o que você esperava. 

Number literals	Example
Decimal		98_222
Hex		0xff
Octal		0o77
Binary		0b1111_0000
Byte(u8 only)	b'A'

### Ponto flutuantes
f32
f64 default
Todos os tipos de ponto flutuante são signed.

### Booleanos
um tipo booleano em Rust possui dois valores possíveis: true e false. Booleanos têm um byte de tamanho.
O tipo booleano em Rust é especificado usando bool.

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### Char
O tipo de Rust char tem quatro bytes de tamanho e representa um valor escalar Unicode, o que significa que pode 
representar muito mais do que apenas ASCII. Letras acentuadas; Caracteres chineses, japoneses e coreanos; emoticons; 
Os valores escalares Unicode variam de U+0000até U+D7FFe U+E000inclusive U+10FFFF
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

## Tipos compostos
Os tipos compostos podem agrupar vários valores em um tipo. Rust tem dois tipos de compostos primitivos: tuplas e arrays.

### Tupla
Uma tupla é uma maneira geral de agrupar vários valores com uma variedade de tipos em um tipo composto. As tuplas têm um comprimento fixo: uma vez declaradas, elas não podem aumentar ou diminuir de tamanho.
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
Para obter os valores individuais de uma tupla, podemos usar a pattern matching para desestruturar um valor de tupla.
```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```
Também podemos acessar um elemento da tupla diretamente usando um ponto final ( .) seguido do índice do valor que queremos acessar.

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
Tal como acontece com a maioria das linguagens de programação, o primeiro índice de uma tupla é 0.

A tupla sem nenhum valor possui um nome especial, unit. Este valor e seu tipo correspondente são escritos () e representam um valor vazio ou um tipo de retorno vazio.

### Array
Cada elemento de um array deve ter o mesmo tipo. Os arrays em Rust têm um comprimento fixo.
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
Arrays são úteis quando você deseja que seus dados sejam alocados na pilha em vez de no heap.
Porém, um array não é tão flexível quanto o tipo Vec. Um vetor é um tipo de coleção semelhante fornecido pela biblioteca padrão que pode aumentar ou diminuir de tamanho. 
```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];
```
Aqui i32 é o tipo de cada elemento. Após o ponto e vírgula, o número 5 indica que o array contém cinco elementos.

Você também pode inicializar um array para conter o mesmo valor para cada elemento especificando o valor inicial, seguido por um ponto e vírgula e, em seguida, o comprimento do array entre colchetes,
```rust
let a = [3; 5];
```
o array aconterá 5 lementos que serão todos definidos com o valor 3. É o mesmo que escrever let a = [3, 3, 3, 3, 3];

O índice do array começa com 0, e pode ser acessado da seguinte maneira:
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
Se você tentar acessar um índice maior ou igual ao tamanho do array, recebrá um erro:  'index out of bounds'.
Rust verificará se o índice especificado é menor que o comprimento do array.
```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```
# Operadores

## Numéricos
+	soma 
-	subtração
*	multiplicação
/	sdivisão
%	resto da divisão

## Relacionais
> 	maior
>=	maior ou igual
<	menor
<=	menor ou igual
==	igual
!=	diferente

## Lógicos
&& 	and
|| 	or
!	not

## Bit a bit
|	or
& 	and
^	
<<	shift a esquerda, deslocamento de bits a esquerda
>>	shift a direita, deslocamento de bits a direita
!	not, inverte os bits

## Atribuição
=
+=
-=
*=
/=
%=

Não tem os operadores de incremento e decremento (++ e --)

# Funções
O código Rust usa Snake Case como estilo convencional para nomes de funções e variáveis, em que todas as letras são minúsculas e sublinham palavras separadas.

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```
Observe que definimos another_function após a main função no código-fonte; poderíamos ter definido isso antes também. Rust não se importa onde você define suas funções, apenas se elas estão definidas em algum lugar em um escopo que pode ser visto pelo chamador.

## Parâmetros
Ao definir vários parâmetros, separe as declarações dos parâmetros com vírgulas,.
```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```
## Statements and Expressions
Os corpos de função são compostos de uma série de instruções que terminam opcionalmente em uma expressão.
Como Rust é uma linguagem baseada em expressões.
* Instruções são instruções que executam alguma ação e não retornam um valor.
* As expressões são avaliadas como um valor resultante.
```rust
fn main() {
    let y = 6;
}
```
let y = 6; é uma instrução de declaração. As instruções não retornam valores. 
As expressões são avaliadas como um valor e constituem a maior parte do restante do código que você escreverá em Rust. Considere uma operação matemática, como 5 + 6, que é uma expressão avaliada como valor 11. As expressões podem fazer parte de instruções.
As expressões podem fazer parte de instruções: instrução let y = 6;é uma expressão que é avaliada como valor 6. Chamar uma função é uma expressão. Chamar uma macro é uma expressão. Um novo bloco de escopo criado com chaves é uma expressão.
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```
As expressões não incluem pontos e vírgulas finais. x+1
Se você adicionar um ponto e vírgula ao final de uma expressão, você a transformará em uma instrução e ela não retornará um valor.

## Funções com valores de retorno
As funções podem retornar valores ao código que as chama. Não nomeamos valores de retorno, mas devemos declarar seu tipo após uma seta ( ->).
Em Rust, o valor de retorno da função é sinônimo do valor da expressão final no bloco do corpo de uma função. Você pode retornar antecipadamente de uma função usando a return.
```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```
Quando uma função não retorna nada, o seu tipo de retorno é unit (), uma tupla vazia.

Podemos retornar mais de uma valor neste caso retoranmos uma tupla.
```rust
fn matriz() -> ([i32;5], usize){
  let arr = [1,2,3,4,5];
  (arr, arr.len())
}

fn main(){
  let (x,y) = matrix();
  println!("{:?} length {}"; x,y);
}

```

# Comentários
// comentário de uma única linha
/* */ comentário de trechos de código ou de várias linhas

## Comentários de documentação
Os comentários de documentação são de uma única linha
//! Comentario para explicar o modulo, explicar a funcionalidade do projeto
/// Isso é um comentario de funcao para explicar o que a funcao faz

# Controle de fluxo

## if/else
Ele avalia se uma expressão booleana é true ou false.
Rust não tentará converter automaticamente tipos não booleanos em booleanos. Você deve ser explícito e sempre fornecer if um booleano como condição. 

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
Como if é uma expressão, podemos usá-la no lado direito de uma let instrução para atribuir o resultado a uma variável, os valores retornados devem ser do mesmo tipo. Se o tipo dos valores retornandos sejam de tipo diferentes o seguite erro ocorre: `if` and `else` have incompatible types.
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

# Laços de repetição

## break
Interrompe a execução de um laço de repetição

## continue
Interrompe a execução do código num laço de repetição e vai para a próxima iteração.

## loop
o loop diz ao Rust para executar um bloco de código repetidamente para sempre ou até que você diga explicitamente para parar.
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
Você pode sair do loop utilizando um break.

### Retornando valores
Podemos retornar um valor com break de dentro de um laço de repetição. Para fazer isso, você pode adicionar o valor que deseja retornar após a expressão break usada para interromper o loop.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

##  Labels (rótulos)
Se você tiver loops dentro de loops, break e  continue são aplicados ao loop mais interno nesse ponto.
Opcionalmente, você pode especificar um rótulo (label) de loop em um loop que pode ser usado com break ou continue. Os rótulos de loop devem começar com aspas simples.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

## while
O laço while é executado enquanto a condição for verdadeira true.
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
## Range
Gera uma faixa de valores.
```rust
fn main() {
  // 3 2 1
  for number in (1..4).rev() {
      println!("{number}");
  }
  println!("{:->20}","-");
  // 1 2 3
  for number in 1..4 {
    println!("{number}");
  }
  println!("{:->20}","-");
  // a b c d
  for letter in 'a'..='d' {
    println!("{letter}");
  }
}
```

## for 
Utilizado quando sabemos o intervalo que o laço será executado. Sabemos o início e o fim da sua execução.
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
Neste exemplo usamos um range e o revertemos para que comece na ordem invertida do 4 até o 1. Como primeiro ele
gera o range para depois ser invertido, o valor 4 não é incluso, para incluir o 4 deve-se usar 1..=4
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
Percorrendo um array pelo índice e valor
```rust
fn main(){
  let ar = [1,2,3,4,5];
  for (i,value) in ar.iter().enumerate(){
    println!("[{}]:{}", i, value);
  }
}
```
# Struct
Um struct , ou estrutura , é um tipo de dados personalizado que permite empacotar e nomear vários valores relacionados que constituem um grupo significativo. Se você estiver familiarizado com uma linguagem orientada a objetos, uma estrutura é como os atributos de dados de um objeto.
As estruturas são semelhantes às tuplas, discutidas na seção “O tipo de tupla” , pois ambas contêm vários valores relacionados. Assim como as tuplas, as partes de uma estrutura podem ser de tipos diferentes. Ao contrário das tuplas, em uma estrutura você nomeará cada dado para que fique claro o que os valores significam.
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
Para usar uma estrutura depois de defini-la, criamos uma instância dessa estrutura especificando valores concretos para cada um dos campos. 
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```
Para obter um valor específico de uma estrutura, usamos a notação de ponto. Se a instância for mutável, podemos alterar um valor usando a notação de ponto e atribuindo a um campo específico.
```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```
Observe que toda a instância deve ser mutável; Rust não nos permite marcar apenas determinados campos como mutáveis.

Como os nomes dos parâmetros e os nomes dos campos struct são exatamente os mesmos no código abixo, podemos usar a sintaxe abreviada do init do campo.
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```
## Criando instâncias de outras instâncias com sintaxe de atualização de estrutura
Muitas vezes é útil criar uma nova instância de uma estrutura que inclua a maioria dos valores de outra instância, mas altere alguns. Você pode fazer isso usando a sintaxe de atualização de struct .
```rust
fn main() {

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
O código acima também cria uma instância user2 que possui um valor diferente para email mas possui os mesmos valores para os campos username, activee sign_in_countde user1. Deve ..user1vir por último.

## Usando estruturas de tupla sem campos nomeados para criar tipos diferentes
Rust também oferece suporte a estruturas semelhantes a tuplas, chamadas tuple structs. As estruturas de tupla têm o significado adicional que o nome da estrutura f
ornece, mas não possuem nomes associados aos seus campos; em vez disso, eles apenas têm os tipos de campos.
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
## Estruturas semelhantes a unidades sem quaisquer campos
Você também pode definir estruturas que não possuem campos! Elas são chamadas de unit-like structs semelhantes a unidades porque se comportam de maneira semelhante a (). 
unit-like structs podem ser úteis quando você precisa implementar uma característica em algum tipo, mas não possui nenhum dado que deseja armazenar no próprio tipo. 
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```
Também é possível que structs armazenem referências a dados pertencentes a outra coisa, mas para isso é necessário o uso de times , um recurso do Rust que discutiremos no Capítulo 10. Lifetimes garantem que os dados referenciados por uma struct sejam válidos por quanto tempo. contanto que a estrutura seja. Digamos que você tente armazenar uma referência em uma estrutura sem especificar tempos de vida, como a seguir; isso não vai funcionar:
```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```
username: &str,
  |       ^ expected named lifetime parameter

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
Refatorando com tuplas
```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```
Refatoração com struct
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```
Quando temos estruturas maiores, é útil ter uma saída um pouco mais fácil de ler; nesses casos, podemos usar {:#?}em vez de {:?}na println!string. 
A dbg!macro pode ser muito útil quando você está tentando descobrir o que seu código está fazendo!
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
## Métodos
Os métodos são semelhantes às funções: nós os declaramos com a fn palavra-chave e um nome, eles podem ter parâmetros e um valor de retorno.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
Escolhemos &sel faqui pelo mesmo motivo que usamos &Rectangle na versão da função: não queremos assumir a propriedade e queremos apenas ler os dados na estrutura, não gravar nele. Se quiséssemos alterar a instância na qual chamamos o método como parte do que o método faz, usaríamos &mut self como primeiro parâmetro.
```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
Quando você chama um método com object.something(), Rust adiciona automaticamente &, &mut, ou *corresponde object à assinatura do método. 

## Métodos com mais parâmetros
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
## Funções Associadas
Todas as funções definidas em um implbloco são chamadas de funções associadas porque estão associadas ao tipo nomeado após o impl.
Podemos definir funções associadas que não possuem selfcomo primeiro parâmetro (e portanto não são métodos) porque não precisam de uma instância do tipo para trabalhar.
Funções associadas que não são métodos são frequentemente usadas para construtores que retornarão uma nova instância da estrutura. Geralmente são chamados de new.
```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
Para chamar esta função associada, usamos a ::sintaxe com o nome da estrutura; let sq = Rectangle::square(3).

## Vários impl blocos
Cada estrutura pode ter vários implblocos. 
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

# Emun
Enums permitem definir um tipo enumerando suas possíveis variantes
Onde as estruturas fornecem uma maneira de agrupar campos e dados relacionados, como a Rectangle com seus width e height, as enumerações fornecem uma maneira de dizer que um valor é um de um conjunto possível de valores.
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
Podemos criar instâncias de cada uma das duas variantes IpAddrKind assim:
```rust
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
```
Observe que as variantes do enum têm namespace sob seu identificador e usamos dois pontos duplos para separar os dois. Isso é útil porque agora ambos os valores IpAddrKind::V4e IpAddrKind::V6são do mesmo tipo: IpAddrKind.
```rust
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```
Podemos colocar dados diretamente em cada variante do enum
```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```
Cada variante pode ter diferentes tipos e quantidades de dados associados. 
```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```
O código abaixo ilustra que você pode colocar qualquer tipo de dado dentro de uma variante enum: strings, tipos numéricos ou estruturas, por exemplo. Você pode até incluir outro enum.  
```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
Este enum possui quatro variantes com tipos diferentes:

* Quit não tem nenhum dado associado a ele.
* Move tem campos nomeados, como uma estrutura faz.
* Write inclui um único String.
* ChangeColor inclui três valores i32.
Definir um enum com variantes como as da Listagem 6-2 é semelhante a definir diferentes tipos de definições de struct.
```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```
Há mais uma semelhança entre enums e structs: assim como podemos definir métodos em structs usando impl, também podemos definir métodos em enums.
```rust
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```
## Option Enum
É outro enum definido pela biblioteca padrão. O Optiontipo codifica o cenário muito comum em que um valor pode ser algo ou nada. Por exemplo, se você solicitar o primeiro item de uma lista não vazia, obterá um valor. Se você solicitar o primeiro item de uma lista vazia, não receberá nada.
Rust não possui o recurso nulo que muitas outras linguagens possuem. Nulo é um valor que significa que não há valor ali.
Rust não possui nulos, mas possui um enum que pode codificar o conceito de um valor presente ou ausente. Este enum é Option<T>e é definido pela biblioteca padrão da seguinte forma:
```rust
enum Option<T> {
    None,
    Some(T),
}
```
O Option<T>enum é tão útil que está incluído no prelúdio; você não precisa trazê-lo explicitamente para o escopo. Suas variantes também estão incluídas no prelúdio: você pode usar Somediretamente Nonesem o Option:: prefixo. O Option<T>enum ainda é apenas um enum regular e Some(T)ainda Nonesão variantes do tipo Option<T>.
```rust
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```
O tipo some_numberé Option<i32>. O tipo de some_charis Option<char>.
Para absent_number, Rust exige que anotemos o Optiontipo geral: o compilador não pode inferir o tipo que a Somevariante correspondente conterá olhando apenas para um Nonevalor. Aqui, dizemos a Rust que pretendemos que absent_numberseja do tipo Option<i32>.
Quando temos um Somevalor, sabemos que um valor está presente e o valor é mantido dentro de Some. Quando temos um Nonevalor, em certo sentido significa a mesma coisa que nulo: não temos um valor válido. 
```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

```
Rust não entende como adicionar an i8e an Option<i8>, porque são tipos diferentes. 
Em outras palavras, você precisa converter an Option<T>em a T antes de poder realizar T operações com ele. 
A match expressão é uma construção de fluxo de controle que faz exatamente isso quando usada com enums: ela executará código diferente dependendo de qual variante da enum possui, e esse código pode usar os dados dentro do valor correspondente.

## match
Rust tem uma construção de fluxo de controle extremamente poderosa chamada matchque permite comparar um valor com uma série de padrões e, em seguida, executar o código com base no padrão correspondente.
O poder matchvem da expressividade dos padrões e do fato de o compilador confirmar que todos os casos possíveis foram tratados.
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
Isso parece muito semelhante a uma expressão condicional usada com if, mas há uma grande diferença: com if, a condição precisa ser avaliada como um valor booleano, mas aqui pode ser de qualquer tipo. O tipo coinneste exemplo é o Coinenum .
Se você quiser executar várias linhas de código em uma expressão match, deverá usar chaves e a vírgula após as chaves será opcional.
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
Como exemplo, vamos alterar uma de nossas variantes de enum para armazenar dados dentro dela.
```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```
Na expressão de correspondência deste código, adicionamos uma variável chamada stateao padrão que corresponde aos valores da variante Coin::Quarter. Quando a Coin::Quartercorresponder, a statevariável será vinculada ao valor do estado daquele trimestre. Então podemos usar stateno código.
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

```
Se fôssemos chamar value_in_cents(Coin::Quarter(UsState::Alaska)), state terá o valor UsState::Alaska.

### Matching com Option<T>
Também podemos lidar com Option<T>using match, como fizemos com Coinenum! Em vez de comparar moedas, compararemos as variantes de Option<T>, mas a forma como a matchexpressão funciona permanece a mesma.
Digamos que queremos escrever uma função que receba an Option<i32>e, se houver um valor dentro, adicione 1 a esse valor. Se não houver um valor dentro, a função deverá retornar o Nonevalor e não tentar realizar nenhuma operação
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```
### Matches são exaustivos
Rust sabe que não cobrimos todos os casos possíveis e até sabe qual padrão esquecemos! As correspondências em Rust são exaustivas : devemos esgotar todas as possibilidades para que o código seja válido. Especialmente no caso de Option<T>, quando Rust nos impede de esquecer de lidar explicitamente com o None
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```
Neste caso ocorre o erro: ^ pattern `None` not covered

### Padrões abrangentes e _ Placeholder
Usando enums, também podemos realizar ações especiais para alguns valores específicos, mas para todos os outros valores, executar uma ação padrão.
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```
Para os dois primeiros padrões, os padrões são os valores literais 3 e 7. Para o último padrão que cobre todos os outros valores possíveis, o padrão é a variável que escolhemos nomear other. O código executado para o match other usa a variável passando-a para a move_player função.
Este código é compilado, mesmo que não tenhamos listado todos os valores possíveis que um u8pode ter, porque o último padrão corresponderá a todos os valores não listados especificamente. Este padrão abrangente atende ao requisito que match deve ser exaustivo. Os padrões são avaliados em ordem.
Rust também tem um padrão que podemos usar quando queremos um default, mas não queremos usar o valor no default: _ é um padrão especial que corresponde a qualquer valor e não se vincula a esse valor.  
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```
#### if let Expressões Condicionais
A if let sintaxe permite combinar ife let de uma maneira menos detalhada para lidar com valores que correspondem a um padrão, ignorando o resto. 
```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
```
Para satisfazer a match expressão, temos que adicionar _ => ()após o processamento apenas uma variante, o que é um código padrão chato de adicionar.
Em vez disso, poderíamos escrever isso de uma forma mais curta usando if let.
```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

```
A sintaxe if let usa um padrão e uma expressão separados por um sinal de igual. Funciona da mesma forma que a match, onde a expressão é dada ao match e o padrão é sua primeira opção.
Nesse caso, o padrão é Some(max) e max liga ao valor dentro de Some. Podemos então utilizar max no corpo do if let bloco da mesma forma que utilizamos max na opção match  correspondente. O código no if let bloco não será executado se o valor não corresponder ao padrão.
Usar if let significa menos digitação, menos recuo e menos código clichê. No entanto, você perde a verificação exaustiva que match impõe. A escolha entre match e if let depende do que você está fazendo em sua situação específica e se obter concisão é uma compensação apropriada para perder uma verificação exaustiva.
Podemos incluir um else com um if let. O bloco de código que acompanha the else é o mesmo bloco de código que acompanha o _ case na match expressão equivalente a if let e else.
```rust
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }


    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

```
Quando os valores enum contêm dados, você pode usar match ou if let para extrair e usar esses valores.

Também é possível misturar e combinar expressões if let, else if e else if let. Fazer isso nos dá mais flexibilidade do que uma match expressão na qual podemos expressar apenas um valor para comparar com os padrões. Além disso, Rust não exige que as condições em uma série de opções if let, else if, else if letse relacionem entre si.
```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```
Essa estrutura condicional nos permite oferecer suporte a requisitos complexos. Com os valores codificados que temos aqui, este exemplo será impresso Using purple as the background color.
Você pode ver que if let também pode introduzir variáveis ​​shadowed (sobreadas) da mesma forma que match opções: a linha if let Ok(age) = age introduz uma nova age variável sombreada que contém o valor dentro da Ok variante. Isso significa que precisamos colocar a if age > 30 condição dentro desse bloco: não podemos combinar essas duas condições em if let Ok(age) = age && age > 30. O sombreado age que queremos comparar com 30 não é válido até que o novo escopo comece com as chaves.
A desvantagem de usar if let expressões é que o compilador não verifica a exaustividade, ao passo que com match expressões o faz. 

### Patterns and Matching
Padrões são uma sintaxe especial em Rust para correspondência com a estrutura de tipos, tanto complexos quanto simples. O uso de padrões em conjunto com match expressões e outras construções oferece mais controle sobre o fluxo de controle de um programa. Um padrão consiste em alguma combinação do seguinte:
* Literals
* Destructured arrays, enums, structs, or tuples
* Variables
* Wildcards
* Placeholders
Alguns exemplos de padrões incluem x, (a, 3)e Some(Color::Red)
Nosso programa então compara os valores com os padrões para determinar se ele tem o formato correto dos dados para continuar executando um determinado trecho de código.
Para usar um padrão, nós o comparamos com algum valor. Se o padrão corresponder ao valor, usamos as partes do valor em nosso código. 

#### match Arms
Formalmente, match as expressões são definidas como a palavra-chave match, um valor para correspondência e um ou mais opções de correspondência que consistem em um padrão e uma expressão a ser executada se o valor corresponder ao padrão dessa opção.
```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```
```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```
Os padrões nesta matchexpressão são Nonee Some(i)à esquerda de cada seta.
Um requisito para matchas expressões é que elas sejam exaustivas , no sentido de que todas as possibilidades para o valor na matchexpressão devem ser consideradas.
Uma maneira de garantir que você cobriu todas as possibilidades é ter um padrão genérico para o última opção: por exemplo, um nome de variável que corresponda a qualquer valor nunca pode falhar e, portanto, cobre todos os casos restantes.
O padrão específico _corresponderá a qualquer coisa, mas nunca se vincula a uma variável, por isso é frequentemente usado no última opção de correspondência. O _ padrão pode ser útil quando você deseja ignorar algum valor não especificado.

#### while let Loops Condicionais
Semelhante em construção a if let, o while let loop condicional permite que um while loop seja executado enquanto um padrão continuar a corresponder.
```rust
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
```
Este exemplo imprime 3, 2 e depois 1. O popmétodo retira o último elemento do vetor e retorna Some(value). Se o vetor estiver vazio, pop retorna None. O while loop continua executando o código em seu bloco enquanto popretornar Some.

#### for Loops
Em um forloop, o valor que segue diretamente a palavra-chave foré um padrão. Por exemplo, no for x in yé xo padrão. 
O Código baixo demonstra como usar um padrão em um for loop para desestruturar ou separar uma tupla como parte do for loop.
```rust
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
```
Adaptamos um iterador usando o enumeratemétodo para que ele produza um valor e o índice desse valor, colocado em uma tupla. O primeiro valor produzido é a tupla (0, 'a'). Quando este valor corresponder ao padrão (index, value), index será 0e valueserá 'a', imprimindo a primeira linha da saída.

#### let Statements
```rust
let x = 5;
```

Cada vez que você usou uma let afirmação como essa, você usou padrões, embora talvez não tenha percebido isso! Mais formalmente, uma let declaração se parece com isto:
```rust
let PATTERN = EXPRESSION;
```
Em instruções como let x = 5; o nome de uma variável no PATTERN slot, o nome da variável é apenas uma forma particularmente simples de padrão. Rust compara a expressão com o padrão e atribui todos os nomes que encontra.
Desestruturar uma tupla.
```rust
    let (x, y, z) = (1, 2, 3);
```
Aqui, comparamos uma tupla com um padrão. Rust compara o valor (1, 2, 3) com o padrão (x, y, z) e vê se o valor corresponde ao padrão, então Rust se liga 1a x, 2a ye 3a z.
Se o número de elementos no padrão não corresponder ao número de elementos na tupla, o tipo geral não corresponderá e obteremos um erro do compilador.
```rust
    let (x, y) = (1, 2, 3);

```
Erro: expected a tuple with 3 elements, found one with 2 elements
Para corrigir o erro, poderíamos ignorar um ou mais valores na tupla usando _ou ..

#### Function Parameters
Os parâmetros de função também podem ser padrões.
```rust
fn foo(x: i32) {
    // code goes here
}
```
 O x é um padrão! Assim como fizemos com let.

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```
Este código é impresso Current location: (3, 5). Os valores &(3, 5) correspondem ao padrão &(x, y), assim xcomo o valor 3 e y o valor 5.

#### Refutabilidade: se um padrão pode não corresponder
Os padrões vêm em duas formas: refutáveis ​​e irrefutáveis. Os padrões que corresponderão a qualquer valor possível passado são irrefutáveis . Um exemplo estaria x na declaração let x = 5;porque x corresponde a qualquer coisa e, portanto, não pode deixar de corresponder. Padrões que podem não corresponder a algum valor possível são refutáveis . Um exemplo estaria Some(x) na expressão if let Some(x) = a_value porque se o valor na a_value variável for None em vez de Some, o Some(x) padrão não corresponderá.
Parâmetros de função, let instruções e for loops só podem aceitar padrões irrefutáveis, porque o programa não pode fazer nada significativo quando os valores não correspondem.
if let e while let aceitam padrões refutáveis ​​e irrefutáveis, mas o compilador alerta contra padrões irrefutáveis ​​porque, por definição, eles se destinam a lidar com possíveis falhas: a funcionalidade de uma condicional está em sua capacidade de funcionar de maneira diferente, dependendo do sucesso ou da falha.
Vejamos um exemplo do que acontece quando tentamos usar um padrão refutável onde Rust requer um padrão irrefutável.
```rust
    let Some(x) = some_option_value;
```
Se some_option_value fosse um None valor, não corresponderia ao padrão Some(x), o que significa que o padrão é refutável. No entanto, a let instrução só pode aceitar um padrão irrefutável porque não há nada válido que o código possa fazer com um None valor. Em tempo de compilação, Rust reclamará que tentamos usar um padrão refutável onde um padrão irrefutável é necessário:
Erro: pattern `None` not covered, note: `let` bindings require an "irrefutable pattern"
Se tivermos um padrão refutável onde um padrão irrefutável é necessário, podemos corrigi-lo alterando o código que usa o padrão: em vez de usar let, podemos usar if let.
```rust
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
```
Se fornecermos if letum padrão que sempre corresponderá, o compilador emitirá um aviso.
```rust
    if let x = 5 {
        println!("{}", x);
    };
```
Rust reclama que não faz sentido usar if letum padrão irrefutável:

#### Sintaxe de padrão
Reunimos toda a sintaxe válida em padrões e discutimos por que e quando você pode querer usar cada um deles.

1. Matching Literals
Pode comparar padrões diretamente com literais.
```rust
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

2. Matching variáveis nomeadas
Variáveis nomeadas são padrões irrefutáveis que correspondem a qualquer valor.
Como match inicia um novo escopo, as variáveis declaradas como parte de um padrão dentro da match expressão irão ocultar aquelas com o mesmo nome fora da match construção, como é o caso com todas as variáveis.
```rust
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
```
Se xtivesse sido um None valor em vez de Some(5), os padrões nos dois primeiros braços não teriam correspondido, então o valor teria correspondido ao sublinhado. Então o x na expressão ainda é o exterior x que não foi sombreado. Neste caso hipotético, match seria impresso Default case, x = None.

O padrão na primeira opção de correspondência não corresponde ao valor definido de x, então o código continua.
O padrão na segunda opção de correspondência introduz uma nova variável chamada y que corresponderá a qualquer valor dentro de um Some valor. Como estamos em um novo escopo dentro da match expressão, esta é uma nova y variável, não a que y declaramos no início com o valor 10. 
Quando a match expressão termina, seu escopo termina, assim como o escopo do inner y.

3. Multiple Patterns
Nas matchexpressões, você pode combinar vários padrões usando a |sintaxe, que é o padrão ou operador.
```rust
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```
4. Matching Ranges of Values with ..=
A ..=sintaxe nos permite fazer a correspondência com um intervalo inclusivo de valores.

```rust
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
```
Se x for 1, 2, 3, 4 ou 5, a primeira opção corresponderá. Essa sintaxe é mais conveniente para vários valores de correspondência do que usar o | operador para expressar a mesma ideia;
os intervalos só são permitidos com char valores numéricos.
```rust
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
```
#### Desestruturando
1. Desestruturando struct
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```
Este código cria as variáveis a​​​​que bcorrespondem aos valores dos campos x e yda pestrutura.
Este exemplo mostra que os nomes das variáveis ​​no padrão não precisam corresponder aos nomes dos campos da estrutura. No entanto, é comum combinar os nomes das variáveis ​​com os nomes dos campos para facilitar a lembrança de quais variáveis ​​vieram de quais campos. Por causa desse uso comum, e porque a escrita let Point { x: x, y: y } = p;
Rust tem uma abreviação para padrões que correspondem a campos struct: você só precisa listar o nome do campo struct, e as variáveis ​​criadas a partir do padrão terão os mesmos nomes.
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```
Este código cria as variáveis x ​​​​e y que correspondem aos campos x e y da p variável. O resultado é que as variáveis x​​e y contêm os valores da struct p.

Também podemos desestruturar com valores literais como parte do padrão struct em vez de criar variáveis ​​para todos os campos. Fazer isso nos permite testar alguns dos campos para valores específicos enquanto criamos variáveis ​​para desestruturar os outros campos.
```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

2. Desestruturando Enums 

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```
Para variantes enum sem quaisquer dados, como Message::Quit, não podemos desestruturar mais o valor. Só podemos combinar o Message::Quit valor literal e nenhuma variável está nesse padrão.

Para variantes de enum do tipo struct, como Message::Move, podemos usar um padrão semelhante ao padrão que especificamos para corresponder às structs.

3. Desestruturando structs e enums aninhados
```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
```
4. Desestruturando structs e tuplas
Podemos misturar, combinar e aninhar padrões de desestruturação de maneiras ainda mais complexas. O exemplo a seguir mostra uma desestruturação complicada onde aninhamos estruturas e tuplas dentro de uma tupla e desestruturamos todos os valores primitivos:
```rust
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

#### Ignorando valores em um padrão
Você viu que às vezes é útil ignorar valores em um padrão, como no último braço de a match, para obter um resumo que na verdade não faz nada, mas leva em conta todos os valores possíveis restantes. Existem algumas maneiras de ignorar valores inteiros ou partes de valores em um padrão: usando o _ padrão (que você viu), usando o _ padrão dentro de outro padrão, usando um nome que comece com um sublinhado ou usando .. para ignorar as partes restantes de um valor.

1. Ignorando um valor inteiro com _
Usamos o sublinhado como um padrão curinga que corresponderá a qualquer valor, mas não será vinculado ao valor.
```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```
Este código irá ignorar completamente o valor 3passado como primeiro argumento e irá imprimir This code only uses the y parameter: 4.
Ignorar um parâmetro de função pode ser especialmente útil em casos em que, por exemplo, você está implementando uma característica quando precisa de uma determinada assinatura de tipo, mas o corpo da função em sua implementação não precisa de um dos parâmetros. Assim, você evita receber um aviso do compilador sobre parâmetros de função não utilizados, como faria se usasse um nome.

2. Ignorando partes de um valor com um aninhado _

Também podemos usar _ dentro de outro padrão para ignorar apenas parte de um valor, por exemplo, quando queremos testar apenas parte de um valor, mas não temos utilidade para as outras partes no código correspondente que queremos executar.

```rust
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
```
Este código será impresso Can't overwrite an existing customized valuee então setting is Some(5). 

Também podemos usar sublinhados em vários lugares dentro de um padrão para ignorar valores específicos. 
```rust
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
```
Este código será impresso Some numbers: 2, 8, 32e os valores 4 e 16 serão ignorados.

3. Ignorando uma variável não utilizada iniciando seu nome com _
Se você criar uma variável, mas não a usar em lugar nenhum, o Rust geralmente emitirá um aviso porque uma variável não 
4. utilizada pode ser um bug. No entanto, às vezes é útil poder criar uma variável que você ainda não usará, como quando
5. você está criando um protótipo ou apenas iniciando um projeto. Nessa situação, você pode dizer ao Rust para não avisar
6. sobre a variável não utilizada iniciando o nome da variável com um sublinhado.
```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```
Observe que há uma diferença sutil entre usar somente _e usar um nome que começa com um sublinhado. A sintaxe _x ainda 
vincula o valor à variável, mas _não vincula de forma alguma.
```rust
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
```
Uma variável não utilizada começando com um sublinhado ainda vincula o valor, que pode assumir a propriedade do valor
Receberemos um erro porque o svalor ainda será movido para _s, o que nos impede de usá-lo snovamente. No entanto, usar o
sublinhado por si só nunca está vinculado ao valor. 
```rust
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
```
será compilada sem erros porque snão será movida para o _.  Usar um sublinhado não vincula o valor
Este código funciona muito bem porque nunca nos vinculamos sa nada; não é movido.

4. Ignorando as partes restantes de um valor com ..
Com valores que possuem muitas partes, podemos usar a ..sintaxe para usar partes específicas e ignorar o restante, 
5. evitando a necessidade de listar sublinhados para cada valor ignorado. O ..padrão ignora quaisquer partes de um valor
6. que não combinamos explicitamente no restante do padrão.
```rust
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

```
tupla
```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```
Neste código, o primeiro e o último valor são combinados com firste last. O .. irá combinar e ignorar tudo no meio.

No entanto, o uso ..deve ser inequívoco. Se não estiver claro quais valores devem ser correspondidos e quais devem ser 
ignorados, Rust nos dará um erro. O código abaixo mostra um exemplo de uso .. ambíguo, portanto não será compilado.
```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```
error: `..` can only be used once per tuple pattern

#### Condicionais Extras com Match Guards
Um match guard é uma if condição adicional, especificada após o padrão em uma opção match, que também deve corresponder
para que esse match seja escolhido. 
A condição pode usar variáveis ​​criadas no padrão. 

```rust
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
```
A desvantagem dessa expressividade adicional é que o compilador não tenta verificar a exaustividade quando expressões de
guarda de correspondência estão envolvidas.

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}
```
O padrão na segunda opção match não introduz uma nova variável y que obscureça o outro y, o que significa que podemos 
usar o outro y na guarda da partida. Em vez de especificar o padrão como Some(y), que teria sombreado o exterior y, especificamos Some(n). Isso cria uma nova variável n que não oculta nada porque não há nvariável fora do arquivo match.
Você também pode usar o operador ou| em um match guard para especificar vários padrões; 
```rust
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
```
A condição de correspondência afirma que o braço corresponde apenas se o valor de x for igual a 4, 5 ou 6 e se y for true.

#### @ Bindings

O operador @ nos permite criar uma variável que contém um valor ao mesmo tempo em que testamos esse valor para uma 
correspondência de padrão.
```rust
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

```
Este exemplo será impresso Found an id in range: 5. Ao especificar id_variable @ antes do intervalo 3..=7, capturamos 
qualquer valor que corresponda ao intervalo e, ao mesmo tempo, testamos se o valor corresponde ao padrão de intervalo.
Usar @ nos permite testar um valor e salvá-lo em uma variável dentro de um padrão.

# Ownership e borrowing check

Ownership é o recurso mais exclusivo do Rust e tem implicações profundas para o restante da linguagem. Ele permite que 
o Rust garanta a segurança da memória sem a necessidade de um coletor de lixo. 
Ownership é um conjunto de regras que regem como um programa Rust gerencia a memória.
Algumas linguagens possuem coleta de lixo que procura regularmente a memória que não é mais utilizada enquanto o 
programa é executado; em outras linguagens, o programador deve alocar e liberar explicitamente a memória. Rust usa uma 
terceira abordagem: a memória é gerenciada por meio de um sistema de propriedade com um conjunto de regras que o 
compilador verifica. Se alguma das regras for violada, o programa não será compilado.
Tanto a pilha quanto o heap são partes da memória disponíveis para seu código usar em tempo de execução, mas são 
estruturados de maneiras diferentes. A pilha armazena valores na ordem em que os obtém e remove os valores na ordem 
oposta. Isso é conhecido como último a entrar, primeiro a sair. Pense em uma pilha de pratos: quando você adiciona 
mais pratos, você os coloca em cima da pilha, e quando precisa de um prato, você tira um de cima.
Todos os dados armazenados na pilha devem ter um tamanho fixo e conhecido. Dados com tamanho desconhecido em tempo de 
compilação ou com tamanho que pode mudar devem ser armazenados no heap.
O heap é menos organizado: quando você coloca dados no heap, você solicita uma certa quantidade de espaço. O alocador de
memória encontra um espaço vazio no heap que seja grande o suficiente, marca-o como em uso e retorna um ponteiro , que é
o endereço desse local.
Esse processo é chamado de alocação no heap e às vezes é abreviado como apenas alocação (enviar valores para a pilha não 
é considerado alocação). Como o ponteiro para o heap tem um tamanho fixo e conhecido, você pode armazenar o ponteiro na
pilha, mas quando quiser os dados reais, deverá desreferenciar o ponteiro.
Enviar para a pilha é mais rápido do que alocar no heap porque o alocador nunca precisa procurar um local para armazenar 
novos dados; esse local está sempre no topo da pilha.
Acessar dados no heap é mais lento do que acessar dados na pilha porque você precisa seguir um ponteiro para chegar lá.
Quando seu código chama uma função, os valores passados ​​para a função (incluindo, potencialmente, ponteiros para
dados no heap) e as variáveis ​​locais da função são colocados na pilha. Quando a função termina, esses valores 
são retirados da pilha.

## O que o Borrow Checker resolve?
* Null pointer exception => quando temos uma variável pointer apontando para um local nulo.
* Segmentation fault => causado pelo null pointer exception, quamdo tentamos desreferenciar uma variável que ter um 
valor nulo.
* Memory leak => quando utilizamos muitos valores alocados na memória heap, e não fazemos a limpeza correta dos valores 
alocados na memória. Mesmo que uma rotina termine os valores permancem alocados na memória. Vazamento de memória.
* Dangling pointers => quando temos pointeiros que não apontam para valor nenhum e tentamos utiliz-alos.
* Double free => quamdo limpamos espaços de memória duas ou mias vezes.
* Use after free -> quando tentamos utilizar ponteiros que apontam para um local da memória que já foi limpa.
* Data races => quando temos um valor mutável e vários processos alterando o mesmo valor ao mesmo tempo. E não tem como
garantir a integridade do dado.

## Regras de Ownership
1. Cada valor tem um dono (owner).
2. Só pode ter um único dono.
3. Quando o dono sai de escopo o valor é limpo.
4. A posse (ownership) pode ser movida para outro dono. A variável que perdeu a posse é invalidada.

Um escopo é o intervalo dentro de um programa para o qual um item é válido.
Já vimos literais de string, onde um valor de string é codificado em nosso programa. 
No caso de uma string literal, conhecemos o conteúdo em tempo de compilação, portanto o texto é codificado diretamente 
no executável final.
Na maioria das linguagens sem GC, é nossa responsabilidade identificar quando a memória não está mais sendo usada e 
chamar o código para liberá-la explicitamente, assim como fizemos para solicitá-la. Fazer isso corretamente tem sido 
historicamente um problema de programação difícil. Se esquecermos, desperdiçaremos memória. Se fizermos isso muito cedo,
teremos uma variável inválida. Se fizermos isso duas vezes, isso também será um bug. Precisamos emparelhar exatamente um
allocatecom exatamente um free.
Rust segue um caminho diferente: a memória é retornada automaticamente quando a variável que a possui sai do escopo. 
Quando uma variável sai do escopo, Rust chama uma função especial para nós. Essa função se chama drop.
Observação: em C++, esse padrão de desalocação de recursos no final da vida útil de um item às vezes é chamado de 
aquisição de recurso é inicialização (RAII - Resource Acquisition Is Initialization) . A drop função em Rust será 
familiar para você se você tiver usado padrões RAII.
```rust
    let x = 5;
    let y = x;
```
Provavelmente podemos adivinhar o que isso está fazendo: “vincular o valor 5a x; em seguida, faça uma cópia do valor xe 
vincule-o a y.” Agora temos duas variáveis, x e y, e ambas iguais 5. Na verdade, é isso que está acontecendo, porque os
números inteiros são valores simples com um tamanho fixo e conhecido, e esses dois 5valores são colocados na pilha.
```rust
    let s1 = String::from("hello");
    let s2 = s1;
```
 A String é composto de três partes, mostradas à esquerda: um ponteiro para a memória que contém o conteúdo da string, 
 um comprimento e uma capacidade.

<img src="img/trpl04-01.svg" width="300"/>

O comprimento é a quantidade de memória, em bytes, que o conteúdo do Stringestá usando atualmente. A capacidade é a 
quantidade total de memória, em bytes, que Stringrecebeu do alocador.
Quando atribuímos s1a s2, os Stringdados são copiados, ou seja, copiamos o ponteiro, o comprimento e a capacidade que 
estão na pilha. Não copiamos os dados no heap ao qual o ponteiro se refere.

<img src="img/trpl04-02.svg" width="300"/>

Representação em memória da variável s2 que possui cópia do ponteiro, comprimento e capacidade do s1

Anteriormente, dissemos que quando uma variável sai do escopo, Rust chama automaticamente a dropfunção e limpa a memória
heap dessa variável. Mas a Figura 4-2 mostra ambos os ponteiros de dados apontando para o mesmo local. Isso é um 
problema: quando s2e s1sair do escopo, ambos tentarão liberar a mesma memória. Isso é conhecido como erro duplo livre e
é um dos bugs de segurança de memória que mencionamos anteriormente. Liberar memória duas vezes pode causar corrupção 
de memória, o que pode levar a vulnerabilidades de segurança.

Para garantir a segurança da memória, após a linha let s2 = s1;, Rust considera s1como inválido. Portanto, Rust não 
precisa liberar nada quando s1sai do escopo. Confira o que acontece quando você tenta usar s1depois s2de criado; não vai
funcionar:
```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
```
Se você ouviu os termos cópia superficial e cópia profunda enquanto trabalhava com outras linguagens, o conceito de 
copiar o ponteiro, o comprimento e a capacidade sem copiar os dados provavelmente soa como fazer uma cópia superficial.
Mas como Rust também invalida a primeira variável, em vez de ser chamada de cópia superficial, ela é conhecida como move.
Neste exemplo, diríamos que s1 foi movido para s2.

<img src="img/trpl04-04.svg" alt="Platform projector." width="300"/>

Isso resolve nosso problema! Com apenas s2válido, quando sair do escopo, ele sozinho liberará a memória e pronto.

Além disso, há uma escolha de design implícita nisso: Rust nunca criará automaticamente cópias “profundas” de seus dados.
Portanto, qualquer cópia automática pode ser considerada barata em termos de desempenho em tempo de execução.

Se quisermos copiar profundamente os dados do heap String, e não apenas os dados da pilha, podemos usar um método comum 
chamado clone.

<img src="img/trpl04-03.svg" width="300"/>

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

Tipos como números inteiros que têm um tamanho conhecido em tempo de compilação são armazenados inteiramente na pilha, 
portanto, cópias dos valores reais são rápidas de serem feitas. Isso significa que não há razão para querermos impedir
x que ela seja válida depois de criarmos a variável y.
```rust
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```
Se um tipo implementa a Copy característica, as variáveis que a utilizam não se movem, mas são copiadas trivialmente, 
tornando-as ainda válidas após a atribuição a outra variável.
Rust não nos permitirá anotar um tipo Copy se o tipo, ou qualquer uma de suas partes, tiver implementado a Drop 
característica.

Como regra geral, qualquer grupo de valores escalares simples pode ser implementado Copy, e nada que exija alocação, 
ou seja, alguma forma de recurso pode ser implementado Copy. Aqui estão alguns dos tipos que implementam Copy:
* Todos os tipos inteiros, como u32.
* O tipo booleano, bool, com valores truee false.
* Todos os tipos de ponto flutuante, como f64.
* O tipo de caractere, char.
* Tuplas, se contiverem apenas tipos que também implementam Copy. Por exemplo, (i32, i32) implementa Copy, mas (i32, String) 
* não implementa.
* Array, * Tuplas, se contiverem apenas tipos que também implementam Copy.

### Funções 
A mecânica de passar um valor para uma função é semelhante àquela de atribuir um valor a uma variável. Passar uma 
variável para uma função irá mover ou copiar, assim como faz a atribuição.
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Retorno de funções
A devolução de valores também pode transferir a propriedade.
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
A propriedade de uma variável sempre segue o mesmo padrão: atribuir um valor a outra variável a move. Quando uma 
variável que inclui dados no heap sai do escopo, o valor será limpo, a drop menos que a propriedade dos dados tenha 
sido movida para outra variável.
Felizmente para nós, Rust possui um recurso para usar um valor sem transferir propriedade, chamado de referências .

## References and Borrowing
Podemos fornecer uma referência ao String valor. Uma referência é como um ponteiro, pois é um endereço que podemos 
seguir para acessar os dados armazenados nesse endereço; esses dados pertencem a alguma outra variável. Ao contrário 
de um ponteiro, é garantido que uma referência aponte para um valor válido de um tipo específico durante a vida dessa 
referência.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
<img src="img/trpl04-05.svg" width="300"/>

Nota: O oposto de referenciar usando &é desreferenciar , que é realizado com o operador de desreferenciar *
A &s1 sintaxe nos permite criar uma referência que se refere ao valor, s1 mas não o possui. Por não ser proprietário, 
o valor para o qual aponta não será descartado quando a referência parar de ser usada.
Da mesma forma, a assinatura da função serve & para indicar que o tipo do parâmetro sé uma referência.
O escopo no qual a variável sé válida é o mesmo de qualquer escopo de parâmetro de função, mas o valor apontado pela 
referência não é descartado quando s deixa de ser usado, pois snão possui propriedade.
Quando as funções têm referências como parâmetros em vez dos valores reais, não precisaremos retornar os valores para
devolver a propriedade, porque nunca tivemos propriedade.
Assim como as variáveis são imutáveis por padrão, as referências também o são. Não temos permissão para modificar algo
ao qual temos referência.

### Referências mutáveis
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
*<b>As referências mutáveis têm uma grande restrição: se você tiver uma referência mutável para um valor, não poderá ter
outras referências para esse valor.</b>*
```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```
Este código que tenta criar duas referências mutáveis falhará s:cannot borrow `s` as mutable more than once at a time.
Este erro indica que este código é inválido porque não podemos emprestar s como mutável mais de uma vez por vez. 
O primeiro empréstimo mutável está incluído r1 e deve durar até ser usado no println!, mas entre a criação dessa 
referência mutável e seu uso, tentamos criar outra referência mutável que r2empresta os mesmos dados do r1.
A vantagem de ter essa restrição é que o Rust pode evitar data races em tempo de compilação.
data races:
* Dois ou mais ponteiros acessam os mesmos dados ao mesmo tempo.
* Pelo menos um dos ponteiros está sendo usado para gravar nos dados.
* Não há nenhum mecanismo sendo usado para sincronizar o acesso aos dados.

Data races causam comportamento indefinido e podem ser difíceis de diagnosticar e corrigir quando você tenta rastreá-las
em tempo de execução; Rust evita esse problema recusando-se a compilar código com data races!
Como sempre, podemos usar chaves para criar um novo escopo, permitindo múltiplas referências mutáveis, mas não simultâneas :
```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```
Rust impõe uma regra semelhante para combinar referências mutáveis e imutáveis.
```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```
Uau! Também não podemos ter uma referência mutável enquanto tivermos uma imutável com o mesmo valor.
Os usuários de uma referência imutável não esperam que o valor mude repentinamente! No entanto, múltiplas referências 
imutáveis são permitidas porque ninguém que está apenas lendo os dados tem a capacidade de afetar a leitura dos dados 
por outra pessoa.
Observe que o escopo de uma referência começa onde ela é introduzida e continua até a última vez que a referência é usada.
Por exemplo, este código será compilado porque o último uso das referências imutáveis, o println!, ocorre antes da 
introdução da referência mutável:
```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```
Os escopos das referências imutáveis r1 terminam r2 após o println! local onde foram usadas pela última vez, que é antes
r3da criação da referência mutável. Esses escopos não se sobrepõem, então esse código é permitido: o compilador pode 
dizer que a referência não está mais sendo usada em um ponto antes do final do escopo.

### Dangling References (Referências pendentes)
Em Rust, o compilador garante que as referências nunca serão referências pendentes: se você tiver uma referência a 
alguns dados, o compilador garantirá que os dados não sairão do escopo antes da referência aos dados.

Vamos tentar criar uma referência pendente para ver como o Rust os evita com um erro em tempo de compilação:
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // expected named lifetime parameter. this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime => fn dangle() -> &'static String {
    let s = String::from("hello");

    &s
}
```
this function's return type contains a borrowed value, but there is no value for it to be borrowed from
```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
Porque sé criado dentro dangle, quando o código danglefor finalizado, sserá desalocado. Mas tentamos retornar uma 
referência a ele. Isso significa que esta referência estaria apontando para invalid String.

A solução aqui é retornar String diretamente:
```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
## Regras de Borrwing
Durante o seu uso:
1. Podemos ter uma única referência caso ela seja mutável.
2. Podemos ter várias referências quando são todas imutáveis.
3. As referências devem ser sempre válidas.

# The Slice Type
Slice permitem fazer referência a uma sequência contígua de elementos em uma coleção, em vez de à coleção inteira. 
Uma fatia é uma espécie de referência, portanto não possui propriedade.

Aqui está um pequeno problema de programação: escreva uma função que pegue uma sequência de palavras separadas por 
espaços e retorne a primeira palavra que encontrar nessa sequência. Se a função não encontrar um espaço na string, 
toda a string deverá ser uma palavra, portanto a string inteira deverá ser retornada.
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```
O primeiro elemento da tupla retornado enumerateé o índice e o segundo elemento é uma referência ao elemento.
```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

## String Slices
Uma slice de string é uma referência a parte de a String e tem a seguinte aparência:
```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```
[starting_index..ending_index], onde starting_indexé a primeira posição na fatia e ending_indexé uma a mais que a
última posição na fatia.
Internamente, a estrutura de dados da slice armazena a posição inicial e o comprimento da slice, que corresponde a 
ending_index menos starting_index. Portanto, no caso de let world = &s[6..11];, world seria uma fatia que contém um 
ponteiro para o byte no índice 6 scom um valor de comprimento de 5.

<img src="img/trpl04-06.svg" width="300"/>

se quiser começar no índice 0, você pode eliminar o valor antes dos dois pontos. Em outras palavras, estes são iguais:
```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```
Da mesma forma, se sua slice incluir o último byte do String, você poderá eliminar o número final. Isso significa que 
são iguais:
```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```
Você também pode eliminar ambos os valores para obter uma slice de toda a string. Então estes são iguais:
```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```
```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
Agora, quando chamamos first_word, obtemos de volta um único valor vinculado aos dados subjacentes. O valor é composto
por uma referência ao ponto inicial da slice e ao número de elementos da slice.

Retornar uma slice também funcionaria para uma second_word função:
```rust
fn second_word(s: &String) -> &str {
```
Agora temos uma API simples que é muito mais difícil de bagunçar porque o compilador garantirá que as referências
permaneçam String válidas.
Os problemas apareceriam mais tarde se continuássemos tentando usar o índice da primeira palavra com uma string vazia.
Os Slices tornam esse bug impossível e nos informam que temos um problema com nosso código muito mais cedo. Usar a 
versão slice first_word gerará um erro em tempo de compilação:
```rust
//cannot borrow `s` as mutable because it is also borrowed as immutable 

fn main() {
    let mut s = String::from("hello world"); 

    let word = first_word(&s); // immutable borrow occurs here

    s.clear(); // error! mutable borrow occurs here

    println!("the first word is: {}", word); // immutable borrow later used here
}
```
Lembre-se das regras de empréstimo que, se tivermos uma referência imutável a algo, não podemos também adotar uma 
referência mutável. Como clear precisa truncar o String, ele precisa obter uma referência mutável. Depois println! da 
chamada to clear usa a referência in word, portanto a referência imutável ainda deve estar ativa nesse ponto. Rust não
permite que a referência mutável clear e a referência imutável word existam ao mesmo tempo, e a compilação falha.


## String Literals como Slices
Lembre-se de que falamos sobre literais de string armazenados dentro do binário. Agora que sabemos sobre fatias, podemos
entender corretamente os literais de string:
```rust
let s = "Hello, world!";
```
O tipo s aqui é &str: é uma slice apontando para aquele ponto específico do binário. É também por isso que os literais 
de string são imutáveis; &str é uma referência imutável.

## String Slices como Parameters
Saber que você pode obter fatias de literais e String valores nos leva a mais uma melhoria no first_word, e essa é a
sua assinatura:
```rust
fn first_word(s: &String) -> &str {
```
Um Rustáceo mais experiente escreveria a assinatura mostrada, porque ela nos permite usar a mesma função tanto em 
&String valores quanto em &str valores.
```rust
fn first_word(s: &str) -> &str {
```
Melhorando a first_word função usando uma fatia de string para o tipo de s parâmetro.

## outros Slices
Assim como podemos querer nos referir a parte de uma string, podemos querer nos referir a parte de um array. Faríamos assim:
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
Esta slice tem o tipo &[i32]. Funciona da mesma maneira que as slice de string, armazenando uma referência ao primeiro 
elemento e um comprimento. Você usará esse tipo de slice para todos os tipos de outras coleções.

# Packages, Crates, and Modules
Os programas que escrevemos até agora estavam em um módulo em um arquivo. À medida que um projeto cresce, você deve 
organizar o código dividindo-o em vários módulos e, em seguida, em vários arquivos.
Rust possui vários recursos que permitem gerenciar a organização do seu código, incluindo quais detalhes são expostos, 
quais detalhes são privados e quais nomes estão em cada escopo dos seus programas. Esses recursos, às vezes chamados 
coletivamente de sistema de módulos , incluem:
* Pacotes: um recurso Cargo que permite construir, testar e compartilhar crates
* Crates: Uma árvore de módulos que produz uma biblioteca ou executável
* Módulos e use: permitem controlar a organização, o escopo e a privacidade dos paths
* Paths: uma forma de nomear um item, como uma estrutura, função ou módulo

## Packages e Crates
Um crate é a menor quantidade de código que o compilador Rust considera por vez.
Mesmo se você executar rustc em vez de cargo passar um único arquivo de código-fonte, o compilador considerará esse 
arquivo como um crate. Os crates podem conter módulos, e os módulos podem ser definidos em outros arquivos que são 
compilados com a crate.
Um crate pode vir em uma de duas formas: um crate binário ou um crate de biblioteca.
Crates binários são programas que você pode compilar em um executável que pode ser executado, como um programa de linha 
de comando ou um servidor. Cada um deve ter uma função chamada main que defina o que acontece quando o executável é 
executado. Todas os crate que criamos até agora foram crates binários.
Crate de biblioteca não têm main função e não são compiladas em um executável. Em vez disso, eles definem funcionalidades
destinadas a serem compartilhadas com vários projetos.
Um package é um bundle de uma ou mais crates que fornece um conjunto de funcionalidades. Um package contém um arquivo 
Cargo.toml que descreve como construir esses crates. Cargo é na verdade um package que contém a crate binário da 
ferramenta de linha de comando que você está usando para construir seu código. O package Cargo também contém um crate 
de biblioteca da qual o crate binário depende.
Um package pode conter quantos crates binários você desejar, mas no máximo apenas um crate de biblioteca. Um package 
deve conter pelo menos um crate, seja uma biblioteca ou um crate binário.
Abra Cargo.toml em seu editor de texto e observe que não há menção a src/main.rs . Cargo segue uma convenção de que 
src/main.rs é a raiz do crate binário com o mesmo nome do package. Da mesma forma, Cargo sabe que se o diretório do 
package contém src/lib.rs , o package contém um crate de biblioteca com o mesmo nome do package, e src/lib.rs é a raiz
do crate. Cargo passa os arquivos raiz do crate para rustc construir a biblioteca ou binário.

```shell
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
Aqui, temos um package que contém apenas src/main.rs , o que significa que contém apenas um crate binário chamado 
my-project. Se um package contém src/main.rs e src/lib.rs , ele possui dois crates: uma binária e uma biblioteca, 
ambas com o mesmo nome do package. Um package pode ter múltiplos crates binárias colocando arquivos no diretório src/bin:
cada arquivo será um crate binário separado.

## Modules
Como os módulos funcionam

* Comece pela crate root : ao compilar um crate, o compilador primeiro procura no arquivo crate root  
(geralmente src/lib.rs para um crate de biblioteca ou src/main.rs para um crate binária) para obter o código a ser compilado.
* Declarando módulos : No arquivo crate root, você pode declarar novos módulos; digamos, você declara um módulo “garden”
com mod garden;. O compilador procurará o código do módulo nestes locais:
Inline, entre colchetes que substituem o ponto e vírgula após mod garden
No arquivo src/garden.rs
No arquivo src/garden/mod.rs
* Declarando submódulos : Em qualquer arquivo que não seja o crate root, você pode declarar submódulos. Por exemplo,
você pode declarar mod vegetables;em src/garden.rs . O compilador procurará o código do submódulo no diretório nomeado
para o módulo pai nestes locais:
Inline, seguindo diretamente mod vegetables, entre colchetes em vez de ponto e vírgula
No arquivo src/garden/vegetables.rs
No arquivo src/garden/vegetables/mod.rs
* Paths para código em módulos : quando um módulo faz parte de seu crate, você pode consultar o código desse módulo de
qualquer outro lugar na mesmo crate, desde que as regras de privacidade permitam, usando o path para o código. 
Por exemplo, um Asparagus tipo no módulo de garden seria encontrado em crate::garden::vegetables::Asparagus.
* Privado vs público : o código dentro de um módulo é privado de seus módulos pai por padrão. Para tornar um módulo 
público, declare-o com pub mod em vez de mod. Para tornar públicos os itens de um módulo público também, use pub antes
de suas declarações.
* use : dentro de um escopo, a use palavra-chave cria atalhos para itens para reduzir a repetição de caminhos longos.
Em qualquer escopo que possa fazer referência a crate::garden::vegetables::Asparagus, você pode criar um atalho com 
use crate::garden::vegetables::Asparagus;e a partir daí basta escrever Asparagus para fazer uso daquele tipo no escopo.

```shell
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │ └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
Filename: src/main.rs
```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```
A pub mod garden;linha diz ao compilador para incluir o código encontrado em src/garden.rs , que é:
Filename: src/garden.rs
```rust
pub mod vegetables;
```
Aqui, pub mod vegetables;significa que o código em src/garden/vegetables.rs também está incluído. Esse código é:
```rust
#[derive(Debug)]
pub struct Asparagus {}
```
Crie uma nova biblioteca chamada restaurant executando cargo new restaurant --lib
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
Definimos um módulo com a mod palavra-chave seguida do nome do módulo (neste caso, front_of_house). O corpo do módulo 
fica entre colchetes. Dentro dos módulos podemos colocar outros módulos, como neste caso com os módulos hostinge serving.
```shell
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
Esta árvore mostra como alguns módulos se aninham uns dentro dos outros; por exemplo, hosting dentro front_of_house. 
A árvore também mostra que alguns módulos são irmãos entre si, o que significa que estão definidos no mesmo módulo; 
hostinge serving são irmãos definidos dentro de front_of_house. Se o módulo A estiver contido no módulo B, dizemos que
o módulo A é filho do módulo B e que o módulo B é o pai do módulo A. Observe que toda a árvore do módulo está enraizada
no módulo implícito denominado crate.
A árvore de módulos pode lembrá-lo da árvore de diretórios do sistema de arquivos em seu computador; esta é uma 
comparação muito adequada!

## Paths para referência a um item na árvore de módulos

Para mostrar ao Rust onde encontrar um item em uma árvore de módulos, usamos um path da mesma forma que usamos um path 
ao navegar em um sistema de arquivos.
Um path pode assumir duas formas:
* Um path absoluto é o path completo começando na crate root; para código de uma crate externo, o path absoluto começa 
* com o nome do crate e, para código do crate atual, começa com o literal crate.
* Um path relativo começa no módulo atual e usa self, super um identificador no módulo atual.

Os caminhos absolutos e relativos são seguidos por um ou mais identificadores separados por dois pontos duplos (::)
Mostraremos duas maneiras de chamar a add_to_waitlist função a partir de uma nova função eat_at_restaurant definida no 
crate root.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
A add_to_waitlist função é definida na mesma caixa que eat_at_restaurant, o que significa que podemos usar a crate 
palavra-chave para iniciar um caminho absoluto. Em seguida, incluímos cada um dos módulos sucessivos até chegarmos ao
add_to_waitlist.

Na segunda vez que chamamos add_to_waitlist, eat_at_restaurant usamos um caminho relativo. O caminho começa com 
front_of_house, o nome do módulo definido no mesmo nível da árvore de módulos que eat_at_restaurant.

Começar com um nome de módulo significa que o caminho é relativo.

Nossa preferência em geral é especificar caminhos absolutos porque é mais provável que desejemos mover definições de 
código e chamadas de itens independentemente umas das outras.

Temos os caminhos corretos para o hosting módulo e a add_to_waitlist função, mas Rust não nos permite usá-los porque não
tem acesso às seções privadas. No Rust, todos os itens (funções, métodos, estruturas, enums, módulos e constantes) são 
privados dos módulos pais por padrão.

Os itens em um módulo pai não podem usar os itens privados dentro dos módulos filhos, mas os itens nos módulos filhos 
podem usar os itens em seus módulos ancestrais. Isso ocorre porque os módulos filhos agrupam e ocultam seus detalhes de
implementação, mas os módulos filhos podem ver o contexto no qual estão definidos. 
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
Tornar o módulo público não torna seu conteúdo público. A pub palavra-chave em um módulo apenas permite que o código
em seus módulos ancestrais se refira a ele, e não acesse seu código interno. 

## Iniciando paths relativos com super
Podemos construir paths relativos que começam no módulo pai, em vez do módulo atual ou no crate root, usando super no
início do path. É como iniciar um path de sistema de arquivos com a .. sintaxe.
```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```
A fix_incorrect_order função está no back_of_hous e módulo, então podemos usar super para ir para o módulo pai de 
back_of_house, que neste caso é crate a raiz. A partir daí, procuramos deliver_order e encontramos. 

## Tornando structs e Enums Públicos
Também podemos usar pub para designar estruturas e enums como públicas, mas há alguns detalhes extras para o uso pub 
com estruturas e enums. Se usarmos pub antes de uma definição de struct, tornaremos a struct pública, mas os campos da
struct ainda serão privados. Podemos tornar cada campo público ou não, caso a caso.
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```
Por outro lado, se tornarmos um enum público, todas as suas variantes serão públicas. 
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```
## Trazendo caminhos para o escopo com a use palavra-chave
Há uma maneira de simplificar esse processo: podemos criar um atalho para um caminho com a use palavra-chave uma vez e 
depois usar o nome mais curto em qualquer outro lugar do escopo
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Trazemos o crate::front_of_house::hosting módulo para o escopo da eat_at_restaurant função, então só precisamos 
especificar hosting::add_to_waitlist para chamar a add_to_waitlist função eat_at_restaurant.
Adicionar use um caminho em um escopo é semelhante a criar um link simbólico no sistema de arquivos.
Ao adicionar use crate::front_of_house::hosting ao crate root, hosting agora é um nome válido nesse escopo, como se o
hosting módulo tivesse sido definido na raiz da caixa. Os caminhos incluídos no escopo use também verificam a 
privacidade, como qualquer outro caminho.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```
O erro do compilador mostra que o atalho não se aplica mais ao customer módulo.
Observe que também há um aviso de que the use não é mais usado em seu escopo! Para corrigir esse problema, mova também
use dentro do customer módulo ou faça referência ao atalho no módulo pai super::hosting dentro do customer módulo filho.

## use Criando caminhos idiomáticos
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```
É a maneira idiomática de trazer uma função para o escopo com use. Trazer o módulo pai da função para o escopo use 
significa que temos que especificar o módulo pai ao chamar a função. Especificar o módulo pai ao chamar a função deixa
claro que a função não está definida localmente, ao mesmo tempo que minimiza a repetição do caminho completo.
Por outro lado, ao trazer structs, enums e outros itens com use, é idiomático especificar o caminho completo. 
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
A exceção a esse idioma é se estivermos trazendo dois itens com o mesmo nome para o escopo com use instruções, porque 
Rust não permite isso. 
```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```
Trazer Result para o escopo dois tipos que possuem o mesmo nome, mas módulos pai diferentes, e como fazer referência a eles.
Como você pode ver, o uso dos módulos pai distingue os dois Resul ttipos. Se, em vez disso, especificássemos use 
std::fmt::Result e use std::io::Result, teríamos dois Result tipos no mesmo escopo e Rust não saberia qual deles nos 
referíamos quando usamos Result.

## Fornecendo novos nomes com a as palavra-chave
Há outra solução para o problema de trazer dois tipos com o mesmo nome para o mesmo escopo use: após o caminho, podemos
especificar as um novo nome local, ou alias , para o tipo.
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## Reexportando nomes com pub use
Quando colocamos um nome no escopo com a use palavra-chave, o nome disponível no novo escopo é privado. Para permitir 
que o código que chama nosso código se refira a esse nome como se tivesse sido definido no escopo desse código, podemos
combinar pub e use. Essa técnica é chamada de reexportação porque estamos trazendo um item para o escopo, mas também 
disponibilizando esse item para que outros o incluam em seu escopo.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Antes dessa mudança, o código externo teria que chamar a add_to_waitlist função usando o path 
restaurant::front_of_house::hosting::add_to_waitlist(). Agora que isso pub usereexportou o hostingmódulo do módulo raiz,
o código externo agora pode usar o caminho restaurant::hosting::add_to_waitlist().

## Usando pacotes externos
Para usar rand em nosso projeto, adicionamos em dependências esta linha ao Cargo.toml :
```toml
rand = "0.8.5"
```
Adicionar rand como uma dependência em Cargo.toml diz ao Cargo para baixar o rand pacote e quaisquer dependências de 
crates.io e randd isponibilizá-lo para nosso projeto.
Então, para trazer rand as definições para o escopo do nosso pacote, adicionamos uma use linha começando com o nome 
da caixa, rand e listamos os itens que queríamos trazer para o escopo.
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```
## Usando caminhos aninhados para limpar use listas grandes
Podemos usar caminhos aninhados para trazer os mesmos itens ao escopo em uma linha.
```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

// Como tmabém podemos usar 
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```
Especificando um caminho aninhado para trazer vários itens com o mesmo prefixo para o escopo.
Podemos usar um caminho aninhado em qualquer nível de um caminho, o que é útil ao combinar duas use instruções que 
compartilham um subcaminho. 
```rust
use std::io;
use std::io::Write;
```
A parte comum desses dois caminhos é std::io, e esse é o primeiro caminho completo. Para mesclar esses dois caminhos em 
uma use instrução, podemos usar self para o caminho aninhado.
```rust
use std::io::{self, Write};
```

## O Operador Glob
Se quisermos trazer todos os itens públicos definidos em um caminho para o escopo, podemos especificar esse caminho 
seguido pelo *operador glob:
```rust
use std::collections::*;
```
Esta use declaração traz todos os itens públicos definidos std::collections para o escopo atual. Tenha cuidado ao usar 
o operador glob! Glob pode tornar mais difícil saber quais nomes estão no escopo e onde um nome usado em seu programa 
foi definido.
O operador glob é frequentemente usado durante testes para trazer tudo que está sendo testado para o tests módulo;

## Separando Módulos em Arquivos Diferentes
Até agora, todos os exemplos neste capítulo definiram vários módulos em um arquivo. Quando os módulos ficam grandes, 
você pode querer mover suas definições para um arquivo separado para facilitar a navegação no código.
Extrairemos os módulos em arquivos em vez de ter todos os módulos definidos no arquivo crate root,
Neste caso, o arquivo crate root é src/lib.rs
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Nome do arquivo: src/front_of_house.rs
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
Observe que você só precisa carregar um arquivo usando uma mod declaração uma vez na árvore do módulo. 
Uma vez que o compilador saiba que o arquivo faz parte do projeto (e saiba onde o código reside na árvore do módulo por 
causa de onde você colocou a mod instrução), 
outros arquivos em seu projeto deverão se referir ao código do arquivo carregado usando um caminho para onde foi declarado,
Em outras palavras, nãomod é uma operação de “inclusão” que você pode ter visto em outras linguagens de programação.
Nome do arquivo: src/front_of_house.rs
```rust
pub mod hosting;
```
Em seguida criamos um diretório src/front_of_house e um arquivo hosting.rs para conter as definições feitas no hosting módulo:
Nome do arquivo: src/front_of_house/hosting.rs
```rust
pub fn add_to_waitlist() {}
```
Se, em vez disso, colocarmos hosting.rs no diretório src , o compilador esperaria que o código hosting.rs estivesse em 
um hosting módulo declarado na raiz da caixa e não declarado como filho do front_of_house módulo.
Até agora, cobrimos os caminhos de arquivo mais idiomáticos que o compilador Rust usa, mas Rust também oferece suporte a
um estilo mais antigo de caminho de arquivo. Para um módulo nomeado front_of_housedeclarado na raiz da caixa, o 
compilador procurará o código do módulo em:

* src/front_of_house.rs (o que abordamos)
* src/front_of_house/mod.rs (estilo antigo, caminho ainda suportado)
Para um módulo chamado hosting que é um submódulo de front_of_house, o compilador procurará o código do módulo em:

* src/front_of_house/hosting.rs (o que abordamos)
* src/front_of_house/hosting/mod.rs (estilo antigo, caminho ainda suportado)

É permitido usar uma mistura de ambos os estilos para módulos diferentes no mesmo projeto, mas pode ser confuso para as 
pessoas que navegam em seu projeto.

A principal desvantagem do estilo que usa arquivos chamados mod.rs é que seu projeto pode acabar com muitos arquivos 
chamados mod.rs , o que pode ficar confuso quando você os abre no editor ao mesmo tempo.
A mod palavra-chave declara módulos, e Rust procura em um arquivo com o mesmo nome do módulo o código que vai para esse módulo.
O código do módulo é privado por padrão, mas você pode tornar as definições públicas adicionando a pub palavra-chave.

# Common Collections
A biblioteca padrão do Rust inclui uma série de estruturas de dados muito úteis chamadas coleções . A maioria dos outros
tipos de dados representa um valor específico, mas as coleções podem conter vários valores. Ao contrário dos tipos de 
matriz e tupla integrados, os dados para os quais essas coleções apontam são armazenados no heap, o que significa que a 
quantidade de dados não precisa ser conhecida em tempo de compilação e pode aumentar ou diminuir à medida que o programa
é executado.

## Vetor
Para criar um novo vetor vazio.
```rust
    let v: Vec<i32> = Vec::new();
```
Os vetores são implementados usando genéricos; Por enquanto, saiba que o Vec<T>tipo fornecido pela biblioteca padrão pode conter qualquer tipo.
Mais frequentemente, você criará um Vec<T>com valores iniciais e o Rust inferirá o tipo de valor que deseja armazenar.
```rust
    let v = vec![1, 2, 3];
```
Como fornecemos i32valores iniciais, Rust pode inferir que o tipo de v is Vec<i32>e a anotação de tipo não é necessária.

### Atualizando um vetor
```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```
Como acontece com qualquer variável, se quisermos alterar seu valor, precisamos torná-la mutável usando a mut palavra-chave.
Os números que colocamos dentro são todos do tipo i32, e Rust infere isso a partir dos dados, então não precisamos da 
Vec<i32> anotação.
### Lendo Elementos de Vetores
```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```
Observe alguns detalhes aqui. Usamos o valor do índice 2para obter o terceiro elemento porque os vetores são indexados 
por número, começando em zero. Usar & e [] nos dá uma referência ao elemento no valor do índice. Quando usamos o get 
método com o índice passado como argumento, obtemos um Option<&T>que podemos usar com match.
A razão pela qual Rust fornece essas duas maneiras de referenciar um elemento é para que você possa escolher como o 
programa se comporta quando você tenta usar um valor de índice fora do intervalo de elementos existentes. 
```rust
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
```


Quando executamos este código, o primeiro []método fará com que o programa entre em pânico porque faz referência a um 
elemento inexistente. Este método é melhor usado quando você deseja que seu programa trave se houver uma tentativa de
acessar um elemento após o final do vetor.
Quando o get método recebe um índice que está fora do vetor, ele retorna None sem entrar em pânico. 

Quando o programa tem uma referência válida, o verificador de empréstimo impõe as regras de propriedade e empréstimo 
para garantir que esta referência e quaisquer outras referências ao conteúdo do vetor permaneçam válidas. 
Lembre-se da regra que afirma que você não pode ter referências mutáveis e imutáveis no mesmo escopo. Essa regra se 
aplica ao código abaixo, onde mantemos uma referência imutável ao primeiro elemento de um vetor e tentamos adicionar um
elemento ao final. Este programa não funcionará se também tentarmos fazer referência a esse elemento posteriormente 
na função:
```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
```
Este erro é devido à forma como os vetores funcionam: como os vetores colocam os valores próximos uns dos outros na 
memória, adicionar um novo elemento no final do vetor pode exigir a alocação de nova memória e a cópia dos elementos
antigos para o novo espaço, se não houver espaço suficiente para colocar todos os elementos próximos uns dos outros
onde o vetor está armazenado atualmente. Nesse caso, a referência ao primeiro elemento apontaria para a memória 
desalocada. As regras de empréstimo evitam que os programas acabem nessa situação.

### Iterando sobre os valores em um vetor
Para acessar cada elemento em um vetor, iteraríamos por todos os elementos em vez de usar índices para acessar um de 
cada vez. Para obter referências imutáveis para cada elemento em um vetor de i32 valores e imprimi-los.
```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```
Também podemos iterar sobre referências mutáveis para cada elemento em um vetor mutável para fazer alterações em todos 
os elementos.
```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```
Para alterar o valor ao qual a referência mutável se refere, temos que usar o *operador de desreferência para chegar ao
valor i antes de podermos usar o += operador.
Iterar sobre um vetor, seja de forma imutável ou mutável, é seguro devido às regras do verificador de empréstimo. 
Se tentássemos inserir ou remover itens nos for corpos do loop, obteríamos um erro de compilador. A referência ao vetor 
que o for loop contém impede a modificação simultânea de todo o vetor.

### Usando um Enum para armazenar vários tipos
Os vetores só podem armazenar valores do mesmo tipo. Isto pode ser inconveniente; definitivamente existem casos de uso 
para a necessidade de armazenar uma lista de itens de diferentes tipos.
Por exemplo, digamos que queremos obter valores de uma linha em uma planilha na qual algumas colunas da linha contêm 
números inteiros, alguns números de ponto flutuante e algumas strings. Podemos definir um enum cujas variantes conterão
os diferentes tipos de valores, e todas as variantes do enum serão consideradas do mesmo tipo: o do enum. 
Então podemos criar um vetor para armazenar esse enum e, em última análise, conter diferentes tipos.
```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```
Rust precisa saber quais tipos estarão no vetor em tempo de compilação para saber exatamente quanta memória no heap será 
necessária para armazenar cada elemento. Também devemos ser explícitos sobre quais tipos são permitidos neste vetor.
Se você não conhece o conjunto completo de tipos que um programa obterá em tempo de execução para armazenar em um vetor,
a técnica enum não funcionará. Em vez disso, você pode usar um objeto trait.

### Eliminar um vetor elimina seus elementos
Como qualquer outro struct, um vetor é liberado quando sai do escopo.
```rust
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```
Quando o vetor é eliminado, todo o seu conteúdo também é eliminado, o que significa que os inteiros que ele contém serão
limpos. O verificador de empréstimo garante que quaisquer referências ao conteúdo de um vetor sejam usadas apenas 
enquanto o próprio vetor for válido.

## Strings
Discutiremos strings no contexto de coleções porque strings são implementadas como uma **coleção de bytes**.
Rust possui apenas um tipo de string na linguagem principal, que é a slice string str que geralmente é vista em sua 
forma emprestada &str. São referências a alguns dados de string codificados em UTF-8 armazenados em outro lugar.
Literais de string, por exemplo, são armazenados no binário do programa e, portanto, são slice de string.
O String tipo, que é fornecido pela biblioteca padrão do Rust em vez de codificado na linguagem principal, é um tipo de
string codificado em UTF-8, que pode ser aumentado, mutável e de propriedade.

As string literais são armazenadas na memória estática, junto do código do programa, num local chamado de code segment.
A memória estática é aquela que é carregada na inicialização do programa, e que lá dentro contém o próprio código do programa
e todas as string literais, e variáveis estáticas.
O compilador pega todas as string literais e coloca junto com o código do programa.
A memória estática é limpa quando o programa finaliza a sua execução.
```rust
    let letter: &str = "A" // 65 -> 0100 0001  1 byte = 8 bits 
    // string slice ou string reference
    // Ela contém [inicio, len]
```
"A" é alocado na memória estática e "letter" na stack. "letter" está alocado na stack frame da função main, quando ela
finalizar "letter" é desalocado junto com a stack frame da função main. "Letter" é armazenado na stack frame da função main
e aponta para a fatia de texto estático "A" que está na memória estática do programa.
As string literias são imutáveis.

### Criando uma nova string
```rust
    let mut s = String::new();
```
Esta linha cria uma nova string vazia chamada s, na qual podemos carregar dados.
Utilizamos o to_string método, que está disponível em qualquer tipo que implemente o Display trait, como fazem os 
literais de string.
```rust
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string(); // faz com que uma String seja criado
```
Também podemos usar a função String::from para criar a String a partir de uma string literal.
```rust
    let s = String::from("initial contents");
```
Lembre-se de que as strings são codificadas em UTF-8, portanto podemos incluir nelas quaisquer dados devidamente codificados.
```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

```rust
    let nome = ['M','a','r','i','a'];
    String::from_iter(nome);
    println!("{nome}");
    
    //coerção explícita
    let s: String = "Maria".into();
    println!("{s}");
    
    let m = "Maria".to_owned(); // cria um String
```
As String também são chamadas de String owned.
### Atualizando uma String
Podemos aumentar a String usando o push_str método para anexar uma slice string.
O push_str método usa uma slice  string porque não queremos necessariamente nos apropriar do parâmetro. 
```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push(' '); // insere um char
    s1.push_str(s2); // insere uma string slice
    println!("s2 is {s2}");
```
Se o push_str método se apropriasse de s2, não poderíamos imprimir seu valor na última linha.
O push método pega um único caractere como parâmetro e o adiciona ao arquivo String.
```rust
    let mut s = String::from("lo");
    s.push('l');
```
### Concatenação com o + Operador ou a format! Macro
```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
A string s3conterá Hello, world!. O motivo s1não é mais válido após a adição, e o motivo pelo qual usamos uma referência
a s2, tem a ver com a assinatura do método que é chamado quando usamos o +operador. O +operador usa o add método, cuja 
assinatura é mais ou menos assim:
```rust
fn add(self, s: &str) -> String {
```
Primeiro, s2 tem um &, o que significa que estamos adicionando uma referência da segunda string à primeira string.
Isso ocorre por causa do s parâmetro na add função: só podemos adicionar a &str a a String; não podemos somar dois String 
valores. Mas espere – o tipo de &s2 é &String, não &str, conforme especificado no segundo parâmetro para add.
A razão pela qual podemos usar &s2 na chamada to add é que o compilador pode forçar o &String argumento em a &str. 
Quando chamamos o add método, Rust usa uma coerção deref , que aqui se transforma &s2 em &s2[..].  
Como add não se apropria do s parâmetro, s2 ainda será válido String após esta operação.
Em segundo lugar, podemos ver na assinatura que add se apropria de self, porque self não possui um &. Isso significa que
s1 será movido para a add chamada e não será mais válido depois disso. Portanto, embora let s3 = s1 + &s2;
pareça que copiará ambas as strings e criará uma nova, esta instrução na verdade se apropria de s1, anexa uma cópia do 
conteúdo de s2 e, em seguida, retorna a propriedade do resultado. 
```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```
Neste ponto, s será tic-tac-toe. Com todos os personagens +e " , é difícil ver o que está acontecendo. Para combinações 
de strings mais complicadas, podemos usar a format!macro:
```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
```
### Indexação em Strings
Em muitas outras linguagens de programação, acessar caracteres individuais em uma string referenciando-os por índice é 
uma operação válida e comum. No entanto, se você tentar acessar partes de uma String sintaxe de indexação usando Rust, 
receberá um erro.
```rust
    let s1 = String::from("hello");
    let h = s1[0]; //String` cannot be indexed by `{integer}`
```
O erro e a nota contam a história: Strings Rust não suportam indexação.
Para evitar retornar um valor inesperado e causar bugs que podem não ser descobertos imediatamente, Rust não compila 
esse código e evita mal-entendidos no início do processo de desenvolvimento.
### Representação Interna
A String é um wrapper sobre a Vec<u8>.
```rust
    let hello = String::from("Hola");
```
Neste caso, len será 4, o que significa que o vetor que armazena a string “Hola” tem 4 bytes de comprimento. Cada uma 
dessas letras ocupa 1 byte quando codificada em UTF-8.
```rust
    let hello = String::from("Здравствуйте");
```
Questionado sobre o comprimento da string, você pode dizer 12. Na verdade, a resposta de Rust é 24: esse é o número de
bytes necessários para codificar “Здравствуйте” em UTF-8, porque cada valor escalar Unicode nessa string ocupa 2 bytes 
de armazenamento. 

### Bytes e valores escalares e clusters de grafemas! Oh meu Deus!
Outro ponto sobre o UTF-8 é que na verdade existem três maneiras relevantes de observar strings da perspectiva de Rust:
como bytes, valores escalares e clusters de grafemas (a coisa mais próxima do que chamaríamos de letras ).
Se olharmos para a palavra Hindi “नमस्ते” escrita na escrita Devanagari, ela é armazenada como um vetor de u8valores 
semelhante a este:
```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,224, 165, 135]
```
São 18 bytes e é como os computadores armazenam esses dados. Se olharmos para eles como valores escalares Unicode, que
são o char tipo de Rust, esses bytes ficarão assim:
```rust
['न', 'म', 'स', '्', 'त', 'े']
```
Existem seis charvalores aqui, mas o quarto e o sexto não são letras: são sinais diacríticos que não fazem sentido por
si só.

### Cortando (slice) Strings
A indexação em uma string geralmente é uma má ideia porque não está claro qual deve ser o tipo de retorno da operação de
indexação de string: um valor de byte, um caractere, um cluster de grafema ou uma fatia de string. Se você realmente 
precisa usar índices para criar fatias de string, Rust pede que você seja mais específico.
Em vez de indexar usando []um único número, você pode usar []um intervalo para criar uma fatia de string contendo bytes
específicos:
```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```
Aqui, sestará um &strque contém os primeiros 4 bytes da string. Anteriormente, mencionamos que cada um desses caracteres
tinha 2 bytes, o que significa que sserá Зд.
Se tentássemos dividir apenas parte dos bytes de um caractere com algo como &hello[0..1], Rust entraria em pânico em 
tempo de execução da mesma forma que se um índice inválido fosse acessado em um vetor:
```shell
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:4:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
Você deve usar intervalos para criar fatias de string com cuidado, pois isso pode travar seu programa.

### Métodos para Iterar Strings
A melhor maneira de operar com pedaços de strings é ser explícito sobre se você deseja caracteres ou bytes. Para valores 
escalares Unicode individuais, use o chars método. Chamar chars“Зд” separa e retorna dois valores do tipo char, e você
pode iterar sobre o resultado para acessar cada elemento:
```rust
for c in "Зд".chars() {
    println!("{c}");
}
```
Este código imprimirá o seguinte:
```shell
З
д
```
Alternativamente, o bytesmétodo retorna cada byte bruto, o que pode ser apropriado para o seu domínio:
```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```
Este código imprimirá os quatro bytes que compõem esta string:
```shell
208
151
208
180
```
Mas lembre-se de que os valores escalares Unicode válidos podem ser compostos de mais de 1 byte.

# Armazenando Chaves com Valores Associados HasMap
O tipo HashMap<K, V> armazena um mapeamento de chaves do tipo K para valores do tipo V usando uma função hashing , que
determina como ele coloca essas chaves e valores na memória.
Os mapas hash são úteis quando você deseja pesquisar dados não usando um índice, como é possível com vetores, mas usando 
uma chave que pode ser de qualquer tipo.

## Criando um novo HashMap
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```
Aqui score terá o valor que está associado ao time Azul, e o resultado será 10. O get método retorna um Option<&V>; 
se não houver valor para essa chave no mapa hash, get retornará None. Este programa lida Option com chamando copied 
para obter an Option<i32> em vez de an Option<&i32> e, em seguida unwrap_or, definindo score como zero se scores não 
houver uma entrada para a chave.
Podemos iterar cada par chave/valor em um mapa hash de maneira semelhante à que fazemos com vetores, usando um for loop:
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```
Este código imprimirá cada par em uma ordem arbitrária:
```shell
Yellow: 50
Blue: 10
```
## HashMap e propriedade
Para tipos que implementam a Copy característica, como i32, os valores são copiados no mapa hash. Para valores próprios
como String, os valores serão movidos e o mapa hash será o proprietário desses valores.
```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```
Não podemos usar as variáveis field_name e field_value depois que elas foram movidas para o mapa hash com a chamada para
insert.
Se inserirmos referências a valores no HashMap, os valores não serão movidos para o HashMap. Os valores para os quais
as referências apontam devem ser válidos pelo menos enquanto o HashMap for válido.

## Atualizando um HashMap
Embora o número de pares de chave e valor possa crescer, cada chave exclusiva pode ter apenas um valor associado a ela 
por vez.

### Substituindo um valor
Se inserirmos uma chave e um valor em um mapa hash e depois inserirmos a mesma chave com um valor diferente, o valor 
associado a essa chave será substituído. 
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```
Este código será impresso {"Blue": 25}. O valor original de 10foi substituído.

### Adicionando uma chave e um valor somente se uma chave não estiver presente
É comum verificar se uma determinada chave já existe no mapa hash com um valor e então executar as seguintes ações: se a
chave existir no mapa hash, o valor existente deve permanecer como está. Se a chave não existir, insira-a e um valor 
para ela.
Os mapas hash possuem uma API especial para isso chamada entry que leva a chave que você deseja verificar como parâmetro.
O valor de retorno do entry método é um enum chamado Entry que representa um valor que pode ou não existir. Digamos que
queremos verificar se a chave do time Amarelo tem um valor associado a ela. Caso contrário, queremos inserir o valor 50,
e o mesmo para a equipe Azul. 
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```
O or_insert método on Entry é definido para retornar uma referência mutável ao valor da Entry chave correspondente, se 
essa chave existir e, caso contrário, insere o parâmetro como o novo valor para esta chave e retorna uma referência 
mutável ao novo valor.
A execução do código imprimirá {"Yellow": 50, "Blue": 10}.
A primeira chamada para entryirá inserir a chave do time Amarelo com o valor 50 porque o time Amarelo ainda não possui 
um valor. A segunda chamada para entry não alterará o mapa hash porque o time Azul já possui o valor 10.

### Atualizando um valor com base no valor antigo
Outro caso de uso comum para mapas hash é procurar o valor de uma chave e atualizá-lo com base no valor antigo.
O código que conta quantas vezes cada palavra aparece em algum texto. Usamos um mapa hash com as palavras como chaves e
incrementamos o valor para controlar quantas vezes vimos essa palavra. Se for a primeira vez que vemos uma palavra, 
primeiro inseriremos o valor 0.
```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```
Este código será impresso {"world": 2, "hello": 1, "wonderful": 1}.
O split_whitespace método retorna um iterador sobre subfatias, separadas por espaços em branco, do valor em text. O 
or_insert método retorna uma referência mutável ( &mut V) ao valor da chave especificada. Aqui armazenamos essa 
referência mutável na count variável, portanto, para atribuir esse valor, devemos primeiro desreferenciar count usando o
asterisco ( *). A referência mutável sai do escopo no final do for loop, portanto todas essas alterações são seguras e
permitidas pelas regras de empréstimo.

### Funções de hash
Por padrão, HashMap usa uma função de hash chamada SipHash que pode fornecer resistência a ataques de negação de serviço
(DoS) envolvendo tabelas hash 1 . Este não é o algoritmo de hash mais rápido disponível, mas vale a pena compensar a
melhor segurança que acompanha a queda no desempenho.
Se você criar o seu código e descobrir que a função hash padrão é muito lenta para seus propósitos, você pode 
mudar para outra função especificando um hasher diferente. Um hasher é um tipo que implementa a BuildHasher característica. 

################################
Dada uma lista de inteiros, use um vetor e retorne a mediana (quando classificado, o valor na posição intermediária) e a moda (o valor que ocorre com mais frequência; um mapa hash será útil aqui) da lista.
Converta strings para porco latino. A primeira consoante de cada palavra é movida para o final da palavra e “ay” é adicionado, então “first” se torna “first-fay”. Palavras que começam com uma vogal têm “feno” adicionado ao final (“maçã” torna-se “feno de maçã”). Tenha em mente os detalhes sobre a codificação UTF-8!
Usando um mapa hash e vetores, crie uma interface de texto para permitir que um usuário adicione nomes de funcionários a um departamento de uma empresa. Por exemplo, “Adicionar Sally à Engenharia” ou “Adicionar Amir às Vendas”. Em seguida, deixe o usuário recuperar uma lista de todas as pessoas de um departamento ou de todas as pessoas da empresa por departamento, classificadas em ordem alfabética.
################################

# Manipulação de erros
Rust agrupa os erros em duas categorias principais: erros recuperáveis e irrecuperáveis. 
Rust ão tem exceções. Em vez disso, possui o tipo Result<T, E>para erros recuperáveis e a panic! macro que interrompe a
execução quando o programa encontra um erro irrecuperável. 





















