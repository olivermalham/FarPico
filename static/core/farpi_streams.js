class FarPiCamera extends FarPiElement {
    setup() {
        this.stream = this.getAttribute("stream");
        this.label = this.innerText;
        this.innerHTML =
            `<img src="${this.stream}" alt="FarPi Camera Stream"/>`
        console.log('FarPiCamera added to page - ' + this.stream);
    }
}
customElements.define('farpi-camera', FarPiCamera);
