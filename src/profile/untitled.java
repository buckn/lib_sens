public class Airport {
	private ArrayList<Flight> allFlights;

	public double getTotalRevenue() {
		int total = 0;
		for (Flight x: allFlights) {
			if (x.getCapacity() < x.getNumPassengers()) {
				total += x.getCapacity() * x.getPrice();
			} else {
				total += x.getNumPassengers() * x.getPrice();
			}
		}
		return total;
	}

	public int updateFlights() {

	}
}