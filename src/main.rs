use std::io;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn clear_terminal() {
    if cfg!(unix) {
        Command::new("clear")
            .status()
            .expect("Falha ao limpar o terminal");
    } else if cfg!(windows) {
        Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .expect("Falha ao limpar o terminal");
    }
}

struct Client {
    saldo: f32,
    divida: f32,
}

impl Client {
    fn get_saldo(&mut self) -> f32 {
        self.saldo
    }

    fn get_divida(&mut self) -> f32 {
        self.divida
    }
    fn sacar(&mut self, din: f32) {
        self.saldo = &self.saldo - din;
    }
    fn depositar(&mut self, din: f32) {
        self.saldo = &self.saldo + din;
    }
    fn emprestar(&mut self, din: f32) {
        self.saldo = &self.saldo + din;
        self.divida = din + (din * 0.28);
    }
    fn pagar_emprestimo(&mut self, din: f32) {
        self.saldo = &self.saldo - din;
        self.divida = self.divida - din;
    }
}

fn convert_to_number(s: &str) -> Result<f32, &str> {
    s.trim()
        .parse::<f32>()
        .map_err(|_| "Erro ao converter a string para número de ponto flutuante")
}

fn deposite(mut c: Client) {
    clear_terminal();
    loop {
        println!("{:-^40}", "DEPÓSITO");

        let banner_dep: &str = "Quanto deseja depositar? ex: 1000.0";
        println!("{}", banner_dep);

        let mut inp_dep = String::new();
        io::stdin()
            .read_line(&mut inp_dep)
            .expect("Erro ao ler a entrada");

        let dep: f32 = match convert_to_number(&inp_dep) {
            Ok(value) => value,
            Err(_) => {
                println!("Entrada inválida. Tente novamente.");
                continue;
            }
        };

        println!("Está depositando R$: {}", dep);
        println!("Deseja mudar?");

        let mut esc = String::new();
        io::stdin()
            .read_line(&mut esc)
            .expect("Erro ao ler a entrada");

        if esc.to_lowercase().trim() == "sim" || esc.to_lowercase().trim() == "s" {
            continue;
        } else if esc.to_lowercase().trim() == "n"
            || esc.to_lowercase().trim() == "nao"
            || esc.to_lowercase().trim() == "não"
        {
            c.depositar(dep);
            println!("Agora você tem R$: {}", c.get_saldo());
            println!("Deseja voltar para o menu principal?");

            let mut esc2 = String::new();
            io::stdin()
                .read_line(&mut esc2)
                .expect("Erro ao ler a entrada");

            if esc2.to_lowercase().trim() == "sim" || esc2.to_lowercase().trim() == "s" {
                clear_terminal();
                break;
            } else if esc2.to_lowercase().trim() == "n"
                || esc2.to_lowercase().trim() == "nao"
                || esc2.to_lowercase().trim() == "não"
            {
                continue;
            }
        }

        break;
    }
}

fn saque(mut c: Client) {
    clear_terminal();
    loop {
        println!("{:-^40}", "SAQUE");

        let banner_saque: &str = "Quanto deseja sacar? ex: 1000.0";

        println!("{}", banner_saque);

        let mut inp_saque = String::new();
        io::stdin()
            .read_line(&mut inp_saque)
            .expect("Erro ao ler a entrada");

        let saque: f32 = match convert_to_number(&inp_saque) {
            Ok(value) => value,
            Err(_) => {
                println!("Entrada inválida. Tente novamente.");
                continue;
            }
        };

        println!("Está sacando R$: {}", saque);
        println!("Deseja mudar?");

        let mut esc = String::new();
        io::stdin()
            .read_line(&mut esc)
            .expect("Erro ao ler a entrada");

        if esc.to_lowercase().trim() == "sim" || esc.to_lowercase().trim() == "s" {
            continue;
        } else if esc.to_lowercase().trim() == "n"
            || esc.to_lowercase().trim() == "nao"
            || esc.to_lowercase().trim() == "não"
        {
            c.sacar(saque);
            println!("Agora você tem R$: {}", c.get_saldo());
            println!("Deseja voltar para o menu principal?");

            let mut esc2 = String::new();
            io::stdin()
                .read_line(&mut esc2)
                .expect("Erro ao ler a entrada");

            if esc2.to_lowercase().trim() == "sim" || esc2.to_lowercase().trim() == "s" {
                clear_terminal();
                println!("Aguarde estamos te levando para o menu principal");
                thread::sleep(Duration::from_secs(5));
                break;
            } else if esc2.to_lowercase().trim() == "n"
                || esc2.to_lowercase().trim() == "nao"
                || esc2.to_lowercase().trim() == "não"
            {
                continue;
            }
        }
    }
}

fn emprestimo(mut c: Client) {
    clear_terminal();
    loop {
        println!("{:-^40}", "EMPRESTIMO");

        let banner_emp: &str = "Quanto deseja de empréstimo? ex: 1000.0";

        println!("{}", banner_emp);

        let mut inp_emp = String::new();
        io::stdin()
            .read_line(&mut inp_emp)
            .expect("Erro ao ler a entrada");

        let emp: f32 = match convert_to_number(&inp_emp) {
            Ok(value) => value,
            Err(_) => {
                println!("Entrada inválida. Tente novamente.");
                continue;
            }
        };

        println!("Está sacando R$: {}", emp);
        println!("Deseja mudar?");

        let mut esc = String::new();
        io::stdin()
            .read_line(&mut esc)
            .expect("Erro ao ler a entrada");

        if esc.to_lowercase().trim() == "sim" || esc.to_lowercase().trim() == "s" {
            continue;
        } else if esc.to_lowercase().trim() == "n"
            || esc.to_lowercase().trim() == "nao"
            || esc.to_lowercase().trim() == "não"
        {
            c.emprestar(emp);
            println!("Agora você tem R$: {}", c.get_saldo());
            println!("E suas dividas são {}", c.get_divida());
            println!("Deseja voltar para o menu principal?");

            let mut esc2 = String::new();
            io::stdin()
                .read_line(&mut esc2)
                .expect("Erro ao ler a entrada");

            if esc2.to_lowercase().trim() == "sim" || esc2.to_lowercase().trim() == "s" {
                clear_terminal();
                println!("Aguarde estamos te levando para o menu principal");
                thread::sleep(Duration::from_secs(5));
                break;
            } else if esc2.to_lowercase().trim() == "n"
                || esc2.to_lowercase().trim() == "nao"
                || esc2.to_lowercase().trim() == "não"
            {
                continue;
            }
        }
    }
}

fn pay_emp(mut c: Client) {
    loop {
        clear_terminal();
        println!("{:-^40}", "PAGANDO EMPRÉSTIMO");
        if c.divida > 0.0 {
            println!("Saldo R${}", c.get_saldo());
            println!("Dividas: R${}", c.get_divida());

            println!("{}", "Quantidade que deseja pagar: R$");

            let mut qtd = String::new();
            io::stdin()
                .read_line(&mut qtd)
                .expect("Erro ao ler a entrada");

            println!("Está pagando R$: {}", qtd);
            println!("Deseja mudar?");

            let mut esc = String::new();
            io::stdin()
                .read_line(&mut esc)
                .expect("Erro ao ler a entrada");

            if esc.to_lowercase().trim() == "sim" || esc.to_lowercase().trim() == "s" {
                continue;
            } else if esc.to_lowercase().trim() == "n"
                || esc.to_lowercase().trim() == "nao"
                || esc.to_lowercase().trim() == "não"
            {
                let din: f32 = match convert_to_number(&qtd) {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Entrada inválida. Tente novamente.");
                        continue;
                    }
                };
                if din > c.divida {
                    println!("Valor é maior que o preço da dívida. Tente novamente em 2 segundos.");
                    thread::sleep(Duration::from_secs(2));
                    continue;
                } else {
                    c.pagar_emprestimo(din);

                    println!("{}", "-".repeat(50));
                    println!("Saldo R${}", c.get_saldo());
                    println!("Dividas: R${}", c.get_divida());
                    println!("{}", "-".repeat(50));

                    println!("Deseja voltar ao menu principal?");
                    let mut esc2 = String::new();
                    io::stdin()
                        .read_line(&mut esc2)
                        .expect("Erro ao ler a entrada");

                    if esc2.to_lowercase().trim() == "sim" || esc2.to_lowercase().trim() == "s" {
                        clear_terminal();
                        println!("Aguarde estamos te levando para o menu principal");
                        thread::sleep(Duration::from_secs(5));
                        main();
                        break;
                    } else if esc2.to_lowercase().trim() == "n"
                        || esc2.to_lowercase().trim() == "nao"
                        || esc2.to_lowercase().trim() == "não"
                    {
                        continue;
                    }
                }
            }
        } else {
            clear_terminal();
            println!("Você não possui dividas");
            println!("Saldo R${}", c.get_saldo());
            println!("Dividas: R${}", c.get_divida());
            println!("{}", "-".repeat(50));
            println!("Aguarde estamos te levando para o menu principal");
            thread::sleep(Duration::from_secs(5));
            main()
        }
    }
}

fn main() {
    clear_terminal();
    loop {
        let c1: Client = Client {
            saldo: 1000.0,
            divida: 500.0,
        };

        println!("{:-^40}", "BANCO DO RUST");

        let mut esc = String::new();

        let banner = "Bem vindo ao Banco do Rust\nO que você deseja fazer? ex:\n\nSacar: Digite saque\n\nDepositar: Digite deposito\n\nEmpréstimo: Digite Emprestimo\n\nPagar empréstimo: pagar\n";

        println!("{}", banner);

        io::stdin()
            .read_line(&mut esc)
            .expect("Error reading input");

        if esc.to_lowercase().trim() == "sacar" || esc.to_lowercase() == "saque" {
            saque(c1);
        } else if esc.to_lowercase().trim() == "deposito" || esc.to_lowercase().trim() == "deposite"
        {
            deposite(c1)
        } else if esc.to_lowercase().trim() == "emprestimo"
            || esc.to_lowercase().trim() == "emprestar"
        {
            emprestimo(c1)
        } else if esc.to_lowercase().trim() == "pagar" || esc.to_lowercase().trim() == "pay" {
            pay_emp(c1);
        }
    }
}