import adt_stuff as adt


def main():
    day = adt.DayOfTheWeek.Friday
    print(f"My favorite day is {day}")

    favorites = [
        adt.ComplexEnum.Int(42),
        adt.ComplexEnum.Float(3.14),
        adt.ComplexEnum.Str("hello world"),
    ]
    for favorite in favorites:
        match favorite:
            case adt.ComplexEnum.Int(i=x):
                print(f"My favorite int is {x}")
            case adt.ComplexEnum.Float(f=x):
                print(f"My favorite float is {x}")
            case adt.ComplexEnum.Str(s=x):
                print(f"My favorite string is {x}")


if __name__ == "__main__":
    main()
