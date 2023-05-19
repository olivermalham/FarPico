
class FarPiNavbar extends FarPiElement {
    setup() {
        this.logo = this.getAttribute("logo");
        this.label = this.innerText;
        this.innerHTML =
            `<div class="navbar bg-primary text-neutral-content h-20">
                <div class="flex-1">
                    <img src="${this.logo}" alt="${this.logo} Logo" class="h-20 p-2"/>
                </div>
                ${this.innerHTML}
            </div>`;
    }
}
customElements.define('farpi-navbar', FarPiNavbar);


class FarPiNavbarButton extends FarPiElement {
    setup() {
        this.logo = this.getAttribute("logo");
        this.classList.add("tab")
    }
}
customElements.define('farpi-navbar-button', FarPiNavbarButton);


class FarPiTabList extends FarPiElement {
    // Tab headers
    setup() {
        this.innerHTML =
            `<div class="mr-3">
                ${this.innerHTML}
            </div>`;
        farPiTabRefresh("0");
    } //tabs tabs-boxed
}
customElements.define('farpi-tab-list', FarPiTabList);


class FarPiTab extends FarPiElement {
    // Container for the contents of each tab
    setup() {
        this.classList.add("pr-3");
        this.tab_number = this.getAttribute("number");
    }
}
customElements.define('farpi-tab', FarPiTab);


class FarPiTabBtn extends FarPiElement {
    // Container for the contents of each tab
    setup() {
        this.tab_number = this.getAttribute("tab");
        this.classList.add("btn", "btn-wide", "h-12", "text-xl")
        this.onclick = this.onclick_handler
    }

    onclick_handler(event) {
        farPiTabRefresh(this.tab_number);
    }
}
customElements.define('farpi-tab-btn', FarPiTabBtn);


function farPiTabRefresh(tab_number){
    // Global function to refresh tab visibility.
    let tabs = document.getElementsByTagName("farpi-tab");
    for (let tab in tabs){
        tabs[tab].hidden = (tabs[tab].tab_number !== tab_number);
    }

    let tab_btns = document.getElementsByTagName("farpi-tab-btn");
    for (let btn =0; btn < tab_btns.length; btn++){
        if (btn == tab_number) {
            tab_btns[btn].classList.add("btn-primary", "btn-active", "border-base-100", "border-4");
        } else {
            tab_btns[btn].classList.remove("btn-primary", "btn-active", "border-base-100", "border-4");
        }
    }
}
