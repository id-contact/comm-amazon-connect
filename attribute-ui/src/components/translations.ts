declare var TRANSLATIONS: { [key: string]: { [key: string]: string } };

const translations: { [key: string]: string } = TRANSLATIONS[navigator.language] || TRANSLATIONS.nl;

export default translations;
