import adt_stuff as adt


def main():
    day = adt.DayOfTheWeek.Friday
    print(f"My favorite day is {day}")

    complex = adt.ComplexEnum.Str("hello!")
    print(f"My favorite complex enum is {complex}")


if __name__ == "__main__":
    main()
