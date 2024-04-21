class SiteLocale extends HTMLElement {
    static LOCALE_CODE = ["af", "ar-SA", "ar-IQ", "ar-EG", "ar-LY", "ar-DZ", "ar-MA", "ar-TN", "ar-OM", "ar-YE", "ar-SY", "ar-JO", "ar-LB", "ar-KW", "ar-AE", "ar-BH", "ar-QA", "eu", "bg", "be", "ca", "zh-TW", "zh-CN", "zh-HK", "zh-SG", "hr", "nl", "nl-BE", "en-US", "en-GB", "en-AU", "en-CA", "en-NZ", "en-IE", "en-ZA", "en-JM", "en-BZ", "en-TT", "et", "fo", "fi", "fr", "fr-BE", "fr-CA", "fr-CH", "fr-LU", "gd", "gd-IE", "de", "de-CH", "de-AT", "de-LU", "de-LI", "hu", "is", "in", "it", "it-CH", "lv", "lt", "mk", "ms", "mt", "no", "pl", "pt-BR", "pt", "ro", "ro-MO", "ru", "ru-MO", "sz", "sr", "sk", "sl", "sb", "es", "es-MX", "es-GT", "es-CR", "es-PA", "es-DO", "es-VE", "es-CO", "es-PE", "es-AR", "es-EC", "es-CL", "es-UY", "es-PY", "es-BO", "es-SV", "es-HN", "es-NI", "es-PR", "sx", "sv", "sv-FI", "th", "tn", "tr", "ve", "vi"];
    static LOCALE_TEXT = {
        "aa": "Afaraf",
        "ab": "аҧсуа бызшәа",
        "ae": "avesta",
        "af": "Afrikaans",
        "ak": "Akan",
        "am": "አማርኛ",
        "an": "aragonés",
        "ar": "اللغة العربية",
        "as": "অসমীয়া",
        "av": "авар мацӀ",
        "ay": "aymar aru",
        "az": "azərbaycan dili",
        "ba": "башҡорт теле",
        "be": "беларуская мова",
        "bg": "български език",
        "bh": "भोजपुरी",
        "bi": "Bislama",
        "bm": "bamanankan",
        "bn": "বাংলা",
        "bo": "བོད་ཡིག",
        "br": "brezhoneg",
        "bs": "bosanski jezik",
        "ca": "Català",
        "ce": "нохчийн мотт",
        "ch": "Chamoru",
        "co": "corsu",
        "cr": "ᓀᐦᐃᔭᐍᐏᐣ",
        "cs": "čeština",
        "cu": "ѩзыкъ словѣньскъ",
        "cv": "чӑваш чӗлхи",
        "cy": "Cymraeg",
        "da": "dansk",
        "de": "Deutsch",
        "dv": "Dhivehi",
        "dz": "རྫོང་ཁ",
        "ee": "Eʋegbe",
        "el": "Ελληνικά",
        "en": "English",
        "eo": "Esperanto",
        "es": "Español",
        "et": "eesti",
        "eu": "euskara",
        "fa": "فارسی",
        "ff": "Fulfulde",
        "fi": "suomi",
        "fj": "Vakaviti",
        "fo": "føroyskt",
        "fr": "Français",
        "fy": "Frysk",
        "ga": "Gaeilge",
        "gd": "Gàidhlig",
        "gl": "galego",
        "gn": "avañe'ẽ",
        "gu": "ગુજરાતી",
        "gv": "Gaelg",
        "ha": "هَوُسَ",
        "he": "עברית",
        "hi": "हिन्दी",
        "ho": "Hiri Motu",
        "hr": "Hrvatski",
        "ht": "Kreyòl ayisyen",
        "hu": "magyar",
        "hy": "Հայերեն",
        "hz": "Otjiherero",
        "ia": "Interlingua",
        "id": "Bahasa Indonesia",
        "ie": "Interlingue",
        "ig": "Asụsụ Igbo",
        "ii": "ꆈꌠ꒿ Nuosuhxop",
        "ik": "Iñupiaq",
        "io": "Ido",
        "is": "Íslenska",
        "it": "Italiano",
        "iu": "ᐃᓄᒃᑎᑐᑦ",
        "ja": "日本語",
        "jv": "basa Jawa",
        "ka": "ქართული",
        "kg": "Kikongo",
        "ki": "Gĩkũyũ",
        "kj": "Kuanyama",
        "kk": "қазақ тілі",
        "kl": "kalaallisut",
        "km": "ខេមរភាសា",
        "kn": "ಕನ್ನಡ",
        "ko": "한국어",
        "kr": "Kanuri",
        "ks": "कश्मीरी",
        "ku": "Kurdî",
        "kv": "коми кыв",
        "kw": "Kernewek",
        "ky": "Кыргызча",
        "la": "latine",
        "lb": "Lëtzebuergesch",
        "lg": "Luganda",
        "li": "Limburgs",
        "ln": "Lingála",
        "lo": "ພາສາ",
        "lt": "lietuvių kalba",
        "lu": "Tshiluba",
        "lv": "latviešu valoda",
        "mg": "fiteny malagasy",
        "mh": "Kajin M̧ajeļ",
        "mi": "te reo Māori",
        "mk": "македонски јазик",
        "ml": "മലയാളം",
        "mn": "Монгол хэл",
        "mo": "Лимба молдовеняскэ",
        "mr": "मराठी",
        "ms": "Bahasa Malaysia",
        "mt": "Malti",
        "my": "ဗမာစာ",
        "na": "Ekakairũ Naoero",
        "nb": "Norsk bokmål",
        "nd": "isiNdebele",
        "ne": "नेपाली",
        "ng": "Owambo",
        "nl": "Nederlands",
        "nn": "Norsk nynorsk",
        "no": "Norsk",
        "nr": "isiNdebele",
        "nv": "Diné bizaad",
        "ny": "chiCheŵa",
        "oc": "occitan",
        "oj": "ᐊᓂᔑᓈᐯᒧᐎᓐ",
        "om": "Afaan Oromoo",
        "or": "ଓଡ଼ିଆ",
        "os": "ирон æвзаг",
        "pa": "ਪੰਜਾਬੀ",
        "pi": "पाऴि",
        "pl": "Polski",
        "ps": "پښتو",
        "pt": "Português",
        "qu": "Runa Simi",
        "rm": "rumantsch grischun",
        "rn": "Ikirundi",
        "ro": "Română",
        "ru": "Русский",
        "rw": "Ikinyarwanda",
        "sa": "संस्कृतम्",
        "sc": "sardu",
        "sd": "सिन्धी",
        "se": "Davvisámegiella",
        "sg": "yângâ tî sängö",
        "sh": "Српскохрватски језик",
        "si": "සිංහල",
        "sk": "slovenčina",
        "sl": "slovenščina",
        "sm": "-",
        "sn": "chiShona",
        "so": "Soomaaliga",
        "sq": "Shqip",
        "sr": "српски језик",
        "ss": "SiSwati",
        "st": "Sesotho",
        "su": "Basa Sunda",
        "sv": "Svenska",
        "sw": "Kiswahili",
        "ta": "தமிழ்",
        "te": "తెలుగు",
        "tg": "тоҷикӣ",
        "th": "ไทย",
        "ti": "ትግርኛ",
        "tk": "Türkmen",
        "tl": "Wikang Tagalog",
        "tn": "Setswana",
        "to": "faka Tonga",
        "tr": "Türkçe",
        "ts": "Xitsonga",
        "tt": "татар теле",
        "tw": "Twi",
        "ty": "Reo Tahiti",
        "ug": "ئۇيغۇرچە‎",
        "uk": "Українська",
        "ur": "اردو",
        "uz": "Ўзбек",
        "ve": "Tshivenḓa",
        "vi": "Tiếng Việt",
        "vo": "Volapük",
        "wa": "walon",
        "wo": "Wollof",
        "xh": "isiXhosa",
        "yi": "ייִדיש",
        "yo": "Yorùbá",
        "za": "Saɯ cueŋƅ",
        "zh": "中文",
        "zu": "isiZulu"
    };

    constructor() {
        super();
        this.attachShadow({mode: "open"});
        this.addEventListener("newSiteLocale", event => {
            this.dataset["locale"] = event.detail["locale"];
        });
    }

    async createOptionLocale(locales) {
        const options = document.createElement("ul");
        options.setAttribute("class", "hidden");
        for (const locale of locales) {
            const option = document.createElement("li");
            option.setAttribute("data-code", locale["code"]);
            option.append(await loadSvg(locale["url"]));
            option.append(createSpan(locale["text"]));
            options.append(option);
        }
        return options;
    }

    async place() {
        const style = `
            :host{
                cursor: pointer;
                position: relative;
            }
            :host > svg{
                width: 30px;
                height: 30px;
                padding: 5px 65px;
            }
            ul{
                padding: 5px;
                margin: 0;
                position: absolute;
                width: 160px;
                height: 190px;
                overflow: auto;
                background-color: var(--bg-color-card);
                border: 1px solid var(--border-color-default);
                border-radius: 5px;
                color: var(--text-color-primary);
                scrollbar-width: none;
                -ms-overflow-style: none;
                overscroll-behavior: contain;
            }
            .hidden{
                display: none;
            }
            li{
                position: relative;
                cursor: pointer;
                width: 150px;
                padding: 0 5px;
                display: inline-flex;
                gap: 5px;
            }
            li::before{
                content: "";
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                opacity: 0;
                transition: opacity 0.3s ease-in-out;
                background-color: rgba(0, 0, 0, 0.5);
                pointer-events: none;
            }
            li:hover::before{
                opacity: 1;
            }
            li svg{
                width: 20px;
                height:20px;
            }
            li span{
                font-size: 12px;
                line-height: 20px;
                white-space: nowrap;
            }
            -webkit-scrollbar {
                width: 0;
                display: none;
            }
        `;
        const locales = SiteLocale.LOCALE_CODE.map(code => {
            const lang = code.match(/([a-z]{2})(-[A-Z]{2})?/)[1];
            const url = `svg/locale/${code}.svg`;
            const text = SiteLocale.LOCALE_TEXT[lang] || "";
            return {
                code, url, text
            }
        });
        this.shadowRoot.append(createFragment([
            createStyle(style),
            await loadSvg(`svg/locale/${this.dataset["locale"]}.svg`),
            await this.createOptionLocale(locales)
        ]));
    }

    action() {
        let svg = this.shadowRoot.querySelector('svg');
        const openOption = () => {
            this.shadowRoot.querySelector("ul").classList.toggle("hidden");
        }
        svg.addEventListener('click', openOption);
        this.shadowRoot.querySelectorAll("ul > li").forEach(li => {
            li.addEventListener('click', event => {
                svg.removeEventListener("click", openOption);
                const option = event.currentTarget.querySelector("svg").cloneNode(true);
                option.addEventListener("click", openOption);
                this.shadowRoot.replaceChild(option, svg);
                svg = option;
                changeLocale(event.currentTarget.dataset["code"]);
                this.shadowRoot.querySelector("ul").classList.toggle("hidden");
            });
        });
    }

    connectedCallback() {
        !this.dataset["locale"] && (this.dataset["locale"] = loadLocale());
        this.place().then(() => {
            this.action();
        });
    }
}

customElements.define("site-locale", SiteLocale);