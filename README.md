## Tabela de conteúdos
- [Tabela de conteúdos](#tabela-de-conteúdos)
- [Introdução](#introdução)
- [Contribuindo](#contribuindo)
- [Objetivo](#objetivo)
- [Hardware e Abstrações](#hardware-e-abstrações)
  - [Componentes usados](#componentes-usados)
  - [Display](#display)
  - [Botões do jogador 1](#botões-do-jogador-1)
  - [Botões do jogador 2](#botões-do-jogador-2)
  - [Diagrama](#diagrama)
- [Como rodar o projeto](#como-rodar-o-projeto)
  - [Requisitos globais](#requisitos-globais)
  - [WSL Only](#wsl-only)
  - [Windows](#windows)
  - [Requisitos locais e executando o projeto](#requisitos-locais-e-executando-o-projeto)

## Introdução

Esse projeto é um estudo de Rust em embarcados (e somente de Rust também, já que sou bem iniciante na linguagem)

Esse projeto foi feito por um iniciante e com o único objetivo de treinar, ele é aberto a qualquer tipo de contribuição, forks, clones e etc.   
Faça o que bem entenderem e não precisa dar os créditos, meu objetivo é somente compartilhar conhecimento e minhas experiências na linguagem e em embarcados.

## Contribuindo

Esse código, como dito acima, foi feito por um completo amador na linguagem e em embarcados, então tem MUITAS coisas a melhorar.

A melhor forma de contribuir é refatorar o código e adicionar novas coisas.   
Peço também pra caso alguem se sinta interessado em refatorar o código e contribuir, que faça isso enviando um Pull Request ou abrindo uma issue e detalhando a sua ideia.   

Caso você faça um PR com uma nova feature ou refatoração, por favor, se possível compartilhe o porque está realizando essa mudança e como chegou nessa solução.   
Como dito acima, o meu maior objetivo com esse projeto é aprender mais sobre Rust, compartilhar conhecimento e inspirar novas pessoas a experimentar tanto Rust como embarcados. 

Caso não seja possível detalhar ou queira somente contribuir sem firulas, será muito bem-vindo de qualquer forma. 

## Objetivo

O projeto é um Pong game, então a ideia é simples. 

Uma bolinha que fica rebatendo nas paredes, caso essa bolinha acerte a parede oposta ao jogador, esse jogador ganha ponto e a bolinha volta ao centro da tela.

## Hardware e Abstrações

Nesse projeto, eu estou usando o Esp32, mais especificamente o Esp32-wroom-32 (NodeMCU Esp32S)

O projeto foi feito sem a lib padrão do rust, ou seja, `no_std`.
Resumidamente, isso significa que nada que está disponível na lib padrão do Rust poderá ser usado nesse projeto, a menos que alguém tenha refeito as funcionalidades de algumas coisas da STD do Rust em outras crates, como o `Alloc` ou `spin`. As  crates citadas implementam mutex, Arc e etc em ambientes `no_std`

Pra montar a base do projeto, foi usado o [esp-template](https://github.com/esp-rs/esp-template), um template da comunidade de Rust embarcado pra criar um projeto pronto e funcionando com todas as crates necessárias pra utilizar a maioria das funcionalidades do Esp32 de forma abstraida.

Pórem, vale ressaltar que no momento em que estou escrevendo isso, eles lançaram uma nova versão da crate `esp-hal` (0.17.0) e `esp-wifi` (0.5.0), que quebra compatibilidade com a versão usada nesse projeto.

### Componentes usados
- 4 (dois) push buttons comuns, com resistor de 1k
- 1 (um) display SSD1306 (o que eu uso é LCD monocromático)

Meus botões são de dois pinos, mas caso o seu botão seja de quatro pinos, que é o mais comum, usem o diagrama que eu montei ali em baixo para se localizar melhor.

### Display
O display está ligado da seguinte forma:
- VCC -> 5V
- GND -> GND
- SCL -> P22 (Pin 22/GPIO 22) (Por padrão, esse é o pino SCL do Esp32)
- SDA -> P21 (Pin 21/GPIO 21) (Por padrão, esse é o pino SDA do Esp32)

O artigo que eu usei pra montar o montar o display na placa foi esse aqui:   
https://www.makerguides.com/how-to-connect-an-i2c-lcd-with-esp32/


### Botões do jogador 1
Eles estão ligados da seguinte forma:  
**Up Button**
- Terminal 1 -> 3V3 
- Terminal 2 -> P14 (Pin 14/GPIO 14)  
  
**Down Button**
- Terminal 1 -> 3V3 
- Terminal 2 -> P12 (Pin 12/GPIO 12)
  

### Botões do jogador 2 
Eles estão ligados da seguinte forma:  
**Up Button**
- Terminal 1 -> 3V3 
- Terminal 2 -> P32 (Pin 32/GPIO 32)  
  
**Down Button**
- Terminal 1 -> 3V3 
- Terminal 2 -> P33 (Pin 33/GPIO 33)

Porque eu usei botões e não alavancas de 2 posições?   
Porque eu não tinha em casa e fiz meio na gambiarra, mas caso queira usar, fique a vontade

### Diagrama
![Diagrama](https://i.imgur.com/E15WDGh.png)

Espero que não tenha ficado tão difícil de entender o diagrama.

## Como rodar o projeto

Eu rodo esse projeto dentro do WSL, então tudo que será ensinado aqui vale pra Linux nativo e WSL, dentro do Windows é praticamente a mesma coisa, só muda um passo extra.

### Requisitos globais

Primeiro, você precisa ter o Rust instalado com a ferramenta rustup   
https://www.rust-lang.org/pt-BR/learn/get-started

Agora precisa instalar as dependências do cargo

**Cargo**
```bash
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash # Optional
```

Você precisa das seguintes libs (Linux)
```bash
# Debian/Ubuntu/etc.
apt-get install libudev-dev
# Fedora
dnf install systemd-devel
# Arch Linux
pacman -Syu base-devel
# a flag "yu" no pacman é opcional, mas evita erros de sync (aconteceu comigo)
```

Depois de ter instalado isso, precisamos instalar o espup, um toolchain para o Esp

```bash
espup install
```

Depois de instalado, vai aparecer um arquivo "export-esp.sh" na sua home, esse arquivo é necessário pra você conseguir usar o as ferramentas acima, ele vai setar as variaveis de ambiente necessárias. 

Pra ativar ele no seu terminal, basta usar o seguinte comando:
```bash
. $HOME/export-esp.sh
```   
É recomandado você colocar esse comando dentro da config do seu terminal para ele ser executado toda vez que um novo é aberto.


Depois, você precisa se certificar que tem instalado o Python3.7 ou superior.

Pra instalar ele, basta executar: 
```bash
# Debian/Ubuntu
sudo apt install python3
#Fedora 
sudo dnf install python3
# Arch 
sudo pacman -Sy python3
```

Pra verificar se o python está instalado no seu sistema, execute o seguinte comando:
```bash
python -V
```

Se aparecer a versão do python superior o 3.7, então está tudo certinho.

### WSL Only
Agora se você usa o WSL como eu, você precisa de um passo extra. 
Por padrão, o WSL não reconhece dispositivos USB conectados na máquina HOST, então você precisa de uma ferramenta chamada `usbipd`

Para fazer o WSL reconhecer o seu Esp32, você precisa seguir as instruções da ferramenta nesse site da Microsoft: https://learn.microsoft.com/pt-br/windows/wsl/connect-usb

É bem simples de instalar e usar, mas caso algum problema ocorra, por favor, abram uma Issue.

### Windows
Se você ainda tem dúvidas ou usa Windows nativo como ambiente de desenvolvimento, por favor, leia as instruções no [The Rust on ESP Book](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)


### Requisitos locais e executando o projeto

Após instalar todas as ferramentas necessárias, podemos clonar o projeto com `Git Clone` e executa-lo.

Pra rodar o projeto, primeiro vamos buildar ele pra ter certeza que está tudo certinho, usando o comando `cargo build`  
Depois disso, podemos rodar em modo debug com `cargo run` ou usando a ferramenta "espflash" que instalamos.  
O Cargo run não roda da mesma maneira que projetos tradicionais Rust, ele vai rodar um comando por baixo que na realidade é esse aqui:

```bash
cargo espflash flash --monitor
```

Caso você queira rodar o projeto em modo Release, em vez do modo Debug (que é o padrão), basta usar o seguinte comando:

```bash
cargo espflash flash --release --monitor
```

Para abrir o monitor serial, use somente o seguinte comando:

```bash
cargo espflash monitor
```

O comando `cargo espflash flash` vai compilar seu projeto e jogar o código dentro do seu Esp32 pra ser executado, o comando `cargo espflash monitor` abre o monitor serial, e caso você queira fazer os dois ao mesmo tmepo, basta rodar `cargo espflash flash --monitor`

Vale a pena dar uma olhada na documentação do espflash caso tenham problemas ou interesse nos outros comandos: https://github.com/esp-rs/espflash/blob/main/cargo-espflash/README.md#usage