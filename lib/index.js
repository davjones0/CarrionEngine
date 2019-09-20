var addon = require('../native');

module.exports = {
    initGame: addon.init_game,
    dispatch: addon.dispatch,
    handleKeyPress: addon.handle_key_press
};
console.log(addon.hello());
