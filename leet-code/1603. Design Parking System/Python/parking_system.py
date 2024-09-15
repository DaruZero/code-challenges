class ParkingSystem:
    small_slots = 0
    medium_slots = 0
    big_slots = 0

    def __init__(self, big_slots, medium_slots, small_slots):
        self.small_slots = small_slots
        self.medium_slots = medium_slots
        self.big_slots = big_slots

    def add_car(self, car_type):
        match car_type:
            case 1: 
                self.big_slots -= 1
                return self.big_slots >= 0
            case 2:
                self.medium_slots -= 1
                return self.medium_slots >= 0
            case 3:
                self.small_slots -= 1
                return self.small_slots >= 0