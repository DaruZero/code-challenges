namespace Design_Parking_System;

public class ParkingSystem
{
    public ParkingSystem(int big, int medium, int small)
    {
        BigSlots = big;
        MediumSlots = medium;
        SmallSlots = small;
    }

    public int BigSlots { get; set; }
    public int MediumSlots { get; set; }
    public int SmallSlots { get; set; }

    public bool AddCar(int carType)
    {
        switch (carType)
        {
            case 1:
                BigSlots--;
                return BigSlots >= 0;
            case 2:
                MediumSlots--;
                return MediumSlots >= 0;
            case 3:
                SmallSlots--;
                return SmallSlots >= 0;
            default: return false;
        }
    }
}