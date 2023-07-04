#include "vehicle_purchase.h"

namespace vehicle_purchase {

    // needs_license determines whether a license is needed to drive a type of vehicle. Only "car" and "truck" require a license.
    bool needs_license(std::string kind){
        if(kind == "car" || kind == "truck") { return true; }
        else { return false; }
    }

    // choose_vehicle recommends a vehicle for selection. It always recommends the vehicle that comes first in lexicographical order.
    std::string choose_vehicle(std::string option1, std::string option2) {
        const std::string suffix = " is clearly the better choice.";

        if(option1.compare(option2) <= 0) { return option1 + suffix; }
        else { return option2 + suffix; }
    }

    // calculate_resell_price calculates how much a vehicle can resell for at a certain age.
    double calculate_resell_price(double original_price, double age) {
        if(age < 3) { return original_price * .80; }
        else if(age >= 3 && age < 10) { return original_price * .70; }
        else { return original_price * .50;}
    }

}  // namespace vehicle_purchase