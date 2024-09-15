using Design_Parking_System;

int bigSlots, mediumSlots, smallSlots;

Console.WriteLine("Insert number of big slots:");
while (!int.TryParse(Console.ReadLine(), out bigSlots))
    Console.WriteLine("Insert a valid number!");

Console.WriteLine("Insert number of medium slots:");
while (!int.TryParse(Console.ReadLine(), out mediumSlots))
    Console.WriteLine("Insert a valid number!");

Console.WriteLine("Insert number of small slots:");
while (!int.TryParse(Console.ReadLine(), out smallSlots))
    Console.WriteLine("Insert a valid number!");

Console.WriteLine("Park one car of type:");
Console.WriteLine("\t1 Big car");
Console.WriteLine("\t2 Medium car");
Console.WriteLine("\t3 Small car");

ParkingSystem parkingSystem = new(bigSlots, mediumSlots, smallSlots);

// Iterate enough time to fill all the parking spots
var i = bigSlots + mediumSlots + smallSlots;
for (; i > 0; i--)
{
    var key = Console.ReadKey().KeyChar;
    Console.WriteLine(key is '1' or '2' or '3'
        ? $"\nCan park car? {parkingSystem.AddCar(int.Parse(key.ToString()))}"
        : "Insert a valid number!");
    Console.WriteLine("Park another car:");
}