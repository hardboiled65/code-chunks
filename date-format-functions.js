// Functions

// toStringPad - Make integer to zero-padded string
Number.prototype.toStringPad = function() {
	var str = this.toString()
	if (str.length < 2) { str = '0' + str }
	return str
}
// toFormattedString - Convert Date instance to formatted string
Date.prototype.toFormattedString = function() {
	return `${this.getFullYear().toString()}-${(this.getMonth() + 1).toStringPad()}-${this.getDate().toStringPad()} ${this.getHours().toStringPad()}:${this.getMinutes().toStringPad()}:${this.getSeconds().toStringPad()} ${this.toString().slice(-4, -1)}`
}
// dateToString - Returns Date object as formatted string or null
var dateToString = function(date) {
	if (date !== null) {
		return date.toFormattedString()
	} else {
		return null
	}
}
