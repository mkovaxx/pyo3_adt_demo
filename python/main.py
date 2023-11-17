from typing import List
import adt_stuff as adt


def main():
    day = adt.DayOfTheWeek.Friday
    print(f"My favorite day is {day}")

    meh = adt.ComplexEnum.Str("lol")
    print(isinstance(meh, adt.ComplexEnum.Str))
    print(isinstance(meh, adt.ComplexEnum))

    favorites: List[adt.ComplexEnum] = [
        adt.ComplexEnum.Int(42),
        adt.ComplexEnum.Float(3.14),
        adt.ComplexEnum.Str("hello world"),
    ]
    for favorite in favorites:
        print(type(favorite))
        print_thing(favorite)


def print_thing(thing: adt.ComplexEnum.Int):
    match thing:
        case adt.ComplexEnum.Int(i=x):
            print(f"My favorite int is {x}")
        case adt.ComplexEnum.Float(f=x):
            print(f"My favorite float is {x}")
        case adt.ComplexEnum.Str(s=x):
            print(f"My favorite string is {x}")
        case _:
            print("I don't know what this is ¯\_(ツ)_/¯")


if __name__ == "__main__":
    main()
