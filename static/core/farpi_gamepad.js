class FarPiGamepad extends FarPiElement {
    // NOTE: This currently assumes there is only one gamepad connected, at index 0.

    // Support up to 8 analogue axis
    axis = [null, null, null, null, null, null, null, null, null];

    setup() {
        this.source = this.getAttribute("source");
        this.sample_period = this.getAttribute("period"); // number of milliseconds between gamepad polling
        if (!this.sample_period) this.sample_period = 1000; // Default to 1 second
        this.classList.add("pr-3");

        // Configure all the axiis that we care about from the inner <axis> tags
        // To reverse the sense of an axis, make the range value negative
        // Dead band is the +/- value limits that get rounded to 0, so noise at the center point is ignored
        // "number" is the axis number, "action" is the FarPi action to call on "source"
        let axis_tags = this.getElementsByTagName("axis");
        for(let axis_no = 0; axis_no < axis_tags.length; axis_no++){
            let axis = new FarPiGamepadAxis;
            axis.range = axis_tags[axis_no].getAttribute("range");
            axis.deadband = axis_tags[axis_no].getAttribute("deadband");
            axis.axis = axis_tags[axis_no].getAttribute("number");
            axis.action = axis_tags[axis_no].getAttribute("action");

            this.axis[axis.axis] = axis;
        }

        // Show inactive icon initially
        this.innerHTML =
            `<div class="flex items-center justify-center text-primary border border-2 border-neutral p-1 rounded-xl w-12 h-12 items-center bg-base-100">
                <svg class="svg-icon" style="width: 100%; height: 100%;vertical-align: middle;overflow: hidden;" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"><path d="M937.770667 411.818667A213.546667 213.546667 0 0 0 725.333333 213.333333H298.666667a212.053333 212.053333 0 0 0-212.778667 202.112c-0.426667 3.370667-0.554667 6.869333-0.554667 10.794667v277.76c0 39.466667 15.914667 77.994667 43.605334 105.642667A150.357333 150.357333 0 0 0 234.666667 853.333333c76.8 0 121.770667-42.837333 178.773333-97.109333 10.069333-9.642667 46.933333-27.562667 65.706667-32.042667 10.666667-2.517333 54.997333-2.517333 65.664 0 18.773333 4.437333 55.637333 22.4 65.749333 32.085334C667.562667 810.496 712.533333 853.333333 789.333333 853.333333c39.466667 0 77.994667-15.914667 105.642667-43.605333A150.357333 150.357333 0 0 0 938.666667 704V426.666667c0-4.053333-0.170667-7.68-0.597334-11.093334l-0.298666-3.754666zM853.333333 704c0 16.938667-6.784 33.450667-18.688 45.354667A64.426667 64.426667 0 0 1 789.333333 768c-40.917333 0-66.346667-22.528-119.893333-73.557333-24.576-23.466667-77.696-46.848-104.96-53.290667-17.749333-4.224-40.490667-4.821333-52.48-4.821333s-34.730667 0.64-52.48 4.821333c-27.264 6.442667-80.384 29.824-104.917333 53.248C301.013333 745.472 275.584 768 234.666667 768c-16.938667 0-33.450667-6.784-45.354667-18.688A64.426667 64.426667 0 0 1 170.666667 704l-0.384-276.394667c0.426667-2.176 0.682667-4.394667 0.768-6.613333A127.018667 127.018667 0 0 1 298.666667 298.666667h426.666666c66.858667 0 122.752 52.266667 127.146667 116.053333 0 2.56 0.213333 5.12 0.426667 5.12 0 2.730667-0.128 4.138667 0.426666 6.826667v277.333333z" fill="currentColor" /><path d="M682.666667 554.666667m-42.666667 0a42.666667 42.666667 0 1 0 85.333333 0 42.666667 42.666667 0 1 0-85.333333 0Z" fill="currentColor" /><path d="M768 469.333333m-42.666667 0a42.666667 42.666667 0 1 0 85.333334 0 42.666667 42.666667 0 1 0-85.333334 0Z" fill="currentColor" /><path d="M682.666667 384m-42.666667 0a42.666667 42.666667 0 1 0 85.333333 0 42.666667 42.666667 0 1 0-85.333333 0Z" fill="currentColor" /><path d="M597.333333 469.333333m-42.666666 0a42.666667 42.666667 0 1 0 85.333333 0 42.666667 42.666667 0 1 0-85.333333 0Z" fill="currentColor" /><path d="M341.333333 469.333333m-85.333333 0a85.333333 85.333333 0 1 0 170.666667 0 85.333333 85.333333 0 1 0-170.666667 0Z" fill="currentColor" /></svg>
            </div>`

        console.log('FarPiGamepad added to page');

        if ('GamepadEvent' in window) {
          window.addEventListener("gamepadconnected", () => this.connect_handler());
          window.addEventListener("gamepaddisconnected", () => this.disconnect_handler());
        }
        window.setInterval(() => this.read_gamepad(), this.sample_period);
    }

    connect_handler(){
        // Only called once a joy pad control is pressed
        console.log("Gamepad connected");
        this.gamepads = navigator.getGamepads();
        console.log(this.gamepads);
        
        // Change the icon to active
        this.innerHTML =
            `<div class="flex items-center justify-center text-neutral border border-2 border-neutral p-1 rounded-xl w-12 h-12 items-center bg-primary">
                <svg class="svg-icon" style="width: 100%; height: 100%;vertical-align: middle;overflow: hidden;" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"><path d="M937.770667 411.818667A213.546667 213.546667 0 0 0 725.333333 213.333333H298.666667a212.053333 212.053333 0 0 0-212.778667 202.112c-0.426667 3.370667-0.554667 6.869333-0.554667 10.794667v277.76c0 39.466667 15.914667 77.994667 43.605334 105.642667A150.357333 150.357333 0 0 0 234.666667 853.333333c76.8 0 121.770667-42.837333 178.773333-97.109333 10.069333-9.642667 46.933333-27.562667 65.706667-32.042667 10.666667-2.517333 54.997333-2.517333 65.664 0 18.773333 4.437333 55.637333 22.4 65.749333 32.085334C667.562667 810.496 712.533333 853.333333 789.333333 853.333333c39.466667 0 77.994667-15.914667 105.642667-43.605333A150.357333 150.357333 0 0 0 938.666667 704V426.666667c0-4.053333-0.170667-7.68-0.597334-11.093334l-0.298666-3.754666zM853.333333 704c0 16.938667-6.784 33.450667-18.688 45.354667A64.426667 64.426667 0 0 1 789.333333 768c-40.917333 0-66.346667-22.528-119.893333-73.557333-24.576-23.466667-77.696-46.848-104.96-53.290667-17.749333-4.224-40.490667-4.821333-52.48-4.821333s-34.730667 0.64-52.48 4.821333c-27.264 6.442667-80.384 29.824-104.917333 53.248C301.013333 745.472 275.584 768 234.666667 768c-16.938667 0-33.450667-6.784-45.354667-18.688A64.426667 64.426667 0 0 1 170.666667 704l-0.384-276.394667c0.426667-2.176 0.682667-4.394667 0.768-6.613333A127.018667 127.018667 0 0 1 298.666667 298.666667h426.666666c66.858667 0 122.752 52.266667 127.146667 116.053333 0 2.56 0.213333 5.12 0.426667 5.12 0 2.730667-0.128 4.138667 0.426666 6.826667v277.333333z" fill="currentColor" /><path d="M682.666667 554.666667m-42.666667 0a42.666667 42.666667 0 1 0 85.333333 0 42.666667 42.666667 0 1 0-85.333333 0Z" fill="currentColor" /><path d="M768 469.333333m-42.666667 0a42.666667 42.666667 0 1 0 85.333334 0 42.666667 42.666667 0 1 0-85.333334 0Z" fill="currentColor" /><path d="M682.666667 384m-42.666667 0a42.666667 42.666667 0 1 0 85.333333 0 42.666667 42.666667 0 1 0-85.333333 0Z" fill="currentColor" /><path d="M597.333333 469.333333m-42.666666 0a42.666667 42.666667 0 1 0 85.333333 0 42.666667 42.666667 0 1 0-85.333333 0Z" fill="currentColor" /><path d="M341.333333 469.333333m-85.333333 0a85.333333 85.333333 0 1 0 170.666667 0 85.333333 85.333333 0 1 0-170.666667 0Z" fill="currentColor" /></svg>
            </div>`
    }

    disconnect_handler(){
      console.log("Gamepad disconnected");
    }

    read_gamepad() {
        let gamepad = navigator.getGamepads()[0];
        if (!gamepad) return;

        for (let i = 0; i < gamepad.axes.length; i++) {
            if (!gamepad.axes[i] || !this.axis[i]) continue;
            let value = gamepad.axes[i] * this.axis[i].range;
            if (value > -this.axis[i].deadband && value < this.axis[i].deadband) value = 0;
            value = value.toFixed(2);
            this.action(this.axis[i].action, `"value":${value}`);
        }
    }
}
customElements.define('farpi-gamepad', FarPiGamepad);

class FarPiGamepadAxis {
    // Simple data class to make the gamepad code a little cleaner
    range = 1;
    deadband = 0;
    axis = null;
    action = "";
}