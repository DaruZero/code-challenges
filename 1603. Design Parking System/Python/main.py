from parking_system import ParkingSystem


def main():

    while True:
        try:
            big_slots = int(input("Insert number of big slots: "))
            medium_slots = int(input("Insert number of medium slots: "))
            small_slots = int(input("Insert number of small slots: "))
        except ValueError:
            print("Insert a valid number")
            continue
        else:
            break

    ps = ParkingSystem(big_slots, medium_slots, small_slots)

    print("Types of cars:")
    print("\t1 Big car")
    print("\t2 Medium car")
    print("\t3 Small car")

    max_slots = big_slots + medium_slots + small_slots

    for _ in range(max_slots):
        while True:
            try:
                car_type = int(input("Park car of type: "))
                if not (car_type == 1 or car_type == 2 or car_type == 3):
                    raise ValueError
            except ValueError:
                print("Insert a valid number")
                continue
            else:
                break

        print("Can park car? " + ps.add_car(car_type).__str__())


if __name__ == '__main__':
    main()
