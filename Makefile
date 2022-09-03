
start1:
	cargo run instruction1.sasm

start2:
	cargo run instruction2.sasm

calc:
	cargo instruments -t time instruction2.sasm


.PHONY: start1, start2