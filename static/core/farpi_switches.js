class FarPiSwitch extends FarPiElement {
    setup() {
        this.source = this.getAttribute("source");
        this.label = this.innerText;
        this.innerHTML =
            `<div class="btn btn-primary w-full toggle_switch flex space-x-4">
                <input type="checkbox" class="toggle toggle-primary bg-primary-content" />
                <span class="label">${this.label}</span>
            </div>`
        this.onclick = this.onclick_handler
        console.log('FarPiSwitch added to page - ' + this.source);
    }

    farPiUpdate(newValue) {
        let switch_indicator = this.getElementsByTagName("input")[0];

        switch_indicator.checked = newValue[this.source].state;
    }

    onclick_handler(event) {
        this.action("action_toggle", "");
    }
}
customElements.define('farpi-switch', FarPiSwitch);


class FarPiButton extends FarPiElement {
    setup() {
        this.source = this.getAttribute("source");
        this.label = this.innerText;

        this.innerHTML =
            `<div class="btn btn-primary w-full">
                <span class="label" class="bg-primary-content">${this.label}</span>
            </div>`
        this.onmousedown = this.onmousedown_handler;
        this.onmouseup = this.onmouseup_handler;
        this.ontouchstart = this.onmousedown_handler;
        this.ontouchend = this.onmouseup_handler;
        console.log('FarPiButton added to page - ' + this.source);
    }

    farPiUpdate(newValue) {
    }

    onmousedown_handler(event) {
        event.preventDefault();
        this.action("action_set", '"value":1');
        console.log('FarPiButton down - ' + this.source);
    }

    onmouseup_handler(event) {
        event.preventDefault();
        this.action("action_set", '"value":0');
        console.log('FarPiButton up - ' + this.source);
    }
}
customElements.define('farpi-button', FarPiButton);


class FarPiKeypad extends FarPiElement {
    setup() {
        this.source = this.getAttribute("source");
        this.label = this.innerText;
        this.classList.add("bg-base-100", "rounded-lg", "border", "border-panel", "p-5")

        this.get_mouse_down();
        this.get_mouse_up();

        this.innerHTML =
            `<div class="flex justify-center w-full">
              <kbd class="btn btn-primary text-xl" id="up">&#9650</kbd>
            </div>
            <div class="flex justify-center gap-24 w-full">
              <kbd class="btn btn-primary text-xl" id="left">&#9668</kbd>
              <kbd class="btn btn-primary text-xl" id="right">&#9658</kbd>
            </div>
            <div class="flex justify-center w-full">
              <kbd class="btn btn-primary text-xl" id="down">&#9660</kbd>
            </div>`

        this.onmousedown = this.onmousedown_handler;
        this.onmouseup = this.onmouseup_handler;
        this.ontouchstart = this.onmousedown_handler;
        this.ontouchend = this.onmouseup_handler;

        console.log('FarPiKeypad added to page - ' + this.source);
    }

    get_mouse_down(){
        let tag = this.getElementsByTagName("mouse_down")[0];
        let action = this.get_action(tag,"left");
        this.action_left_down = action[0];
        this.action_left_arg_down = action[1];

        action = this.get_action(tag,"right");
        this.action_right_down = action[0];
        this.action_right_arg_down = action[1];

        action = this.get_action(tag,"up");
        this.action_up_down = action[0];
        this.action_up_arg_down = action[1];

        action = this.get_action(tag,"down");
        this.action_down_down = action[0];
        this.action_down_arg_down = action[1];
    }

    get_mouse_up(){
        let tag = this.getElementsByTagName("mouse_up")[0];
        let action = this.get_action(tag,"left");
        this.action_left_up = action[0];
        this.action_left_arg_up = action[1];

        action = this.get_action(tag,"right");
        this.action_right_up = action[0];
        this.action_right_arg_up = action[1];

        action = this.get_action(tag,"up");
        this.action_up_up = action[0];
        this.action_up_arg_up = action[1];

        action = this.get_action(tag,"down");
        this.action_down_up = action[0];
        this.action_down_arg_up = action[1];
    }

    get_action(tag, button_name){
        let button_tag = tag.getElementsByTagName(button_name)[0];
        return [button_tag.getAttribute("action"), button_tag.innerText];
    }

    farPiUpdate(newValue) {
    }

    onmousedown_handler(event) {
        event.preventDefault();
        switch (event.target.id) {
            case "left":
                console.log('FarPiKeypad left down');
                this.action(this.action_left_down, `"value":${this.action_left_arg_down}`);
            break;

            case "right":
                console.log('FarPiKeypad right down');
                this.action(this.action_right_down, `"value":${this.action_right_arg_down}`);
            break;

            case "up":
                console.log('FarPiKeypad up down');
                this.action(this.action_up_down, `"value":${this.action_up_arg_down}`);
            break;

            case "down":
                console.log('FarPiKeypad down down');
                this.action(this.action_down_down, `"value":${this.action_down_arg_down}`);
            break;
        }
    }

    onmouseup_handler(event) {
        event.preventDefault();
        switch (event.target.id) {
            case "left":
                console.log('FarPiKeypad left up');
                this.action(this.action_left_up, `"value":${this.action_left_arg_up}`);
            break;

            case "right":
                console.log('FarPiKeypad right up');
                this.action(this.action_right_up, `"value":${this.action_right_arg_up}`);
            break;

            case "up":
                console.log('FarPiKeypad up up');
                this.action(this.action_up_up, `"value":${this.action_up_arg_up}`);
            break;

            case "down":
                console.log('FarPiKeypad down up');
                this.action(this.action_down_up, `"value":${this.action_down_arg_up}`);
            break;
        }
    }
}
customElements.define('farpi-keypad', FarPiKeypad);
