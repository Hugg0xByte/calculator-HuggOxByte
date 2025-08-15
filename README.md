# 🧮 Calculator-HuggOxByte

Uma biblioteca de calculadora simples e eficiente escrita em Rust, desenvolvida como projeto de estudo para aprender a linguagem.

## 📋 Sobre o Projeto

Este projeto implementa operações matemáticas básicas usando tipos `u32` (inteiros sem sinal de 32 bits), demonstrando conceitos fundamentais de Rust como:
- Módulos e organização de código
- Funções públicas e privadas
- Tratamento de casos especiais (divisão por zero, subtração com resultado negativo)
- Testes unitários e de integração
- Estrutura de projeto com Cargo

## 🚀 Funcionalidades

### Operações Básicas (`calc1.rs`)
- **Soma** (`add`): Adição simples de dois números
- **Subtração** (`sub`): Subtração com proteção contra resultado negativo (retorna 0)

### Operações Avançadas (`calc2.rs`)
- **Multiplicação** (`multiply`): Multiplicação de dois números
- **Divisão** (`rate`): Divisão com proteção contra divisão por zero (retorna 0)

## 🛠️ Como Usar

### Pré-requisitos
- Rust instalado (versão 1.70+)
- Cargo (gerenciador de pacotes do Rust)

### Instalação e Execução

1. **Clone o repositório:**
```bash
git clone https://github.com/HuggOxByte/calculator-HuggOxByte.git
cd calculator-HuggOxByte
```

2. **Execute o programa principal:**
```bash
cargo run
```

3. **Execute os testes:**
```bash
cargo test
```

4. **Compile o projeto:**
```bash
cargo build
```

### Uso como Biblioteca

```rust
use calculator_HuggOxByte::calc1::{add, sub};
use calculator_HuggOxByte::calc2::{multiply, rate};

fn main() {
    let resultado_soma = add(10, 5);        // 15
    let resultado_sub = sub(10, 3);         // 7
    let resultado_mult = multiply(4, 6);    // 24
    let resultado_div = rate(20, 4);        // 5
    
    println!("Resultados: {}, {}, {}, {}", 
             resultado_soma, resultado_sub, resultado_mult, resultado_div);
}
```

## 📁 Estrutura do Projeto

```
calculator-HuggOxByte/
├── src/
│   ├── lib.rs          # Biblioteca principal e módulos
│   ├── calc1.rs        # Operações básicas (soma, subtração)
│   ├── calc2.rs        # Operações avançadas (multiplicação, divisão)
│   └── main.rs         # Programa de exemplo
├── tests/
│   └── integration_testes.rs  # Testes de integração
├── Cargo.toml          # Configurações e dependências
└── README.md           # Este arquivo
```

## 🧪 Testes

### Testes Unitários
Os testes unitários estão integrados nos módulos principais:
```bash
cargo test
```

### Testes de Integração
Testes separados para verificar a integração entre módulos:
```bash
cargo test --test integration_testes
```

## 🔧 Desenvolvimento

### Adicionando Novas Funcionalidades
1. Crie funções no módulo apropriado (`calc1.rs` ou `calc2.rs`)
2. Adicione testes unitários
3. Atualize os testes de integração se necessário
4. Execute `cargo test` para verificar

### Estrutura de Módulos
- **`calc1.rs`**: Operações matemáticas básicas
- **`calc2.rs`**: Operações matemáticas mais complexas
- **`lib.rs`**: Exporta todos os módulos para uso externo

## 📝 Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 👨‍💻 Autor

**HuggOxByte** - [GitHub](https://github.com/HuggOxByte) - huggo.oliveira1@gmail.com

## 🤝 Contribuições

Contribuições são bem-vindas! Sinta-se à vontade para:
- Reportar bugs
- Sugerir novas funcionalidades
- Enviar pull requests
- Melhorar a documentação

## 📚 Recursos de Aprendizado

Este projeto foi criado para estudar:
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---

⭐ **Se este projeto foi útil para você, considere dar uma estrela no repositório!**
