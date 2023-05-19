class FarPiLED extends FarPiElement {
    setup() {
        this.source = this.getAttribute("source");
        this.label = this.innerText;
        this.innerHTML =
            `<div class="btn btn-outline btn-primary w-full toggle_switch flex space-x-4 no-animation">
                <input type="radio" class="radio radio-primary" />
                <span class="label">${this.label}</span>
            </div>`
        this.onclick = this.onclick_handler
        console.log('FarPiLED added to page - ' + this.source);
    }

    farPiUpdate(newValue) {
        let led_element = this.getElementsByTagName("input")[0];

        led_element.checked = newValue[this.source].state;
    }
}

class FarPiGaugeRound extends FarPiElement {
    setup() {
        this.value = 0;

        this.units = this.getAttribute("units");
        if(!this.units) this.units = "";

        // If no scale attribute is provided, assume the value is in the range 0-1, convert to percentage
        this.scale = this.getAttribute("scale");
        if(this.scale){
            this.scale = parseFloat(this.scale);
        } else {
            this.scale = 100.0;
        }

        // Handle X.Y notation
        this.source = this.getAttribute("source");
        let source_parts = this.source.split(".");
        if(source_parts.length > 1){
            this.source = source_parts[0];
            this.param = source_parts[1];
        } else {
            this.param = "state";
        }

        this.label = this.innerText;

        this.innerHTML =
            `<div class="grid grid-rows-1 justify-items-center place-content-center rounded-lg border border-panel p-5 bg-base-100">
                <span class="radial-progress text-primary border-4 border-neutral-800 bg-neutral-800" style="--value:0;">${this.value}</span>
                <span class="label p-0">${this.label}</span>
            </div>`
        this.onclick = this.onclick_handler
        console.log('FarPiGaugeRound added to page - ' + this.source);
    }

    farPiUpdate(newValue) {
        let gauge_element = this.getElementsByClassName("radial-progress")[0];
        this.value = (newValue[this.source][this.param]*this.scale).toFixed(1);

        gauge_element.style = `--value:${this.value}`;
        gauge_element.innerText = `${this.value}${this.units}`
    }
}


customElements.define('farpi-led', FarPiLED);
customElements.define('farpi-guage-round', FarPiGaugeRound);
