class Employee {private String name;private double salary;
    public Employee(String name, double salary) {this.name = name;this.salary = salary;}
    public void printSalarySlip() {System.out.println("Salary Slip for " + name + ": " + salary);}
    public void saveToDatabase() {// Code to save employee details to the database}
}