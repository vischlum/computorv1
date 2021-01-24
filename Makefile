NAME	= computorv1

all: $(NAME)

$(NAME):
	@cargo build --release
	@cp target/release/computor .

check:
	@cargo check

format:
	@cargo fmt

clean:
	@cargo clean
	
fclean:	clean
	@rm -f computor

re: fclean all

tests:
	cargo run --release -- -h
	cargo run --release -- "12 * X^0  = 12 * X^1"
	cargo run --release -- "14*X^0 -6*X^2 =- 13*X^1"
	cargo run --release -- "14*X^0 -6*X^2 =- 13*X^1" -v
	cargo run --release -- "14*X^0 -6*X^2 =- 13*X^1" -p 0.1
	cargo run --release -- "42 * X^0 = 42 * X^0"
	cargo run --release -- "5 * X^0 + 4 * X^1 - 93 * X^2 = 1 * X^0"
	cargo run --release -- "5 * X^0 + 2 * X^1 + 23.3 * X^2 = 1.4 * X^0" -v