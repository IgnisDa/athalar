from pathlib import Path

import athalar
import typer

app = typer.Typer()

BINARY = "athalar"


@app.command()
def info():
    """Get information about the current project"""
    return 0


generate_help = (
    f"The path where the {BINARY} project is present, defaults to $PWD"
)


@app.command()
def generate(
    path: str = typer.Argument(Path().absolute(), help=generate_help)
):
    """Generate the bindings for the given directory"""
    print(athalar.sum_as_string(2, 6))
    return 0


def main():
    app()


if __name__ == "__main__":
    main()
