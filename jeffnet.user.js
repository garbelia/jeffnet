// ==UserScript==
// @name         jeffnet
// @namespace    http://tampermonkey.net/
// @version      0.1.2
// @description  Accompanying userscript for the JEFF N-day suite
// @author       Garbelia
// @match        *://*.nationstates.net/*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=nationstates.net
// @include      */jnday_sheet.html
// @require      https://craig.global.ssl.fastly.net/js/mousetrap/mousetrap.min.js?a4098
// @grant        GM_getValue
// @grant        GM_setValue
// @grant        window.close
// ==/UserScript==

// CHANGE
var faction = 4 // your faction id
var target =  459 // faction id of your target faction
var targetNum = 520 // number of nations in your target faction
var turbo = false // TURBO MODE??
var EconFocus = "N"
// var EconFocus = "S" // If you would like to produce shields with economic focus nations, move the double slashes at the start of this line to the one above

// DON'T CHANGE
var focus = 0
var links = 6
var oldTarget = -1
var oldFaction = -1
var oldTargetNum = -1

const stratl = document.querySelector("#strat").rows.length;
const econl = document.querySelector("#econ").rows.length;
const intell = document.querySelector("#intel").rows.length;


function inHref(str) {
    return window.location.href.includes(str);
}

function onProductionPage() {
    return inHref("page=nukes/view=production");
}

function onSheet() {
    return inHref("jnday_sheet.html");
}

function nname() {
    return document.body.attributes[1].value;
}

function moveFocus() {
    document.querySelectorAll('a')[focus].style.color = "black";

    if (focus < document.querySelectorAll('a').length - links - 2) {
        focus += links;
    }
    document.querySelectorAll('a')[focus].style.color = "red";
    document.querySelectorAll('a')[focus + 1].scrollIntoView();
}

function numberFromIndicator(indicator) {
    return document.querySelector(indicator).innerText.split("\n")[0]
}

function update() {
    console.log("update")
    if ((oldTargetNum != targetNum && onSheet())) {
        oldTargetNum = targetNum;
        console.log("hi")
        var targetNums = Array.from(document.querySelectorAll('a')).filter(a => a.textContent === "target");
        targetNums.forEach(function (link) {
            link.href = link.href.replace(/start=\d+/, "start=" + Math.max(Math.floor(Math.random() * (targetNum - 50)),0));
        });
    }
    console.log(oldTargetNum + " " + targetNum)

    if ((oldTarget != target && onSheet())) {
        oldTarget = target;
        var targets = Array.from(document.querySelectorAll('a')).filter(a => a.textContent === "target");
        targets.forEach(function (link) {
            link.href = link.href.replace(/fid=\d+/, "fid=" + target);
        });
    }

    if ((oldFaction != faction && onSheet())) {
        oldFaction = faction;
        var factions = Array.from(document.querySelectorAll('a')).filter(a => a.textContent === "join");
        factions.forEach(function (link) {
            link.href = link.href.replace(/fid=\d+/, "fid=" + faction);
        });
        factions = Array.from(document.querySelectorAll('a')).filter(a => a.textContent === "incoming");
        factions.forEach(function (link) {
            link.href = link.href.replace(/fid=\d+/, "fid=" + faction);
        }
        );
    }
}

(function () {
    'use strict';
    if (onSheet()) {
        document.querySelectorAll('a')[focus].style.color = "red";
        update();
    } else if (turbo) {
        if (onProductionPage()) {
            var prod = document.querySelector(".nukestat-production").innerText
            if (prod === "0\nPRODUCTION" || prod==="1\nPRODUCTION") {
               window.close();
            }
        } else if (inHref("incoming")) {
            var message = document.querySelector("p[class=info]");
            var error =document.querySelector(".error");
            if (message != null && !message.innerText.includes("completely")||(error!=null&&document.querySelector(".error").innerText.includes("none left"))) {
              window.close();
            }
        } else if (inHref("page=nukes")&&document.querySelector("p[class=info]")!=null) {
            var num = document.querySelector("span[class=nukeselfview] > a").innerText
            if (num == "0\nNUKES") {
               window.close();
            }
        } else if (inHref("view=targets")) {
            var icon = document.querySelector("button[class='button big icon danger']");
            if (icon == null) {
                window.close();
            }
        } else if (inHref("join_faction")) {
            var message = document.querySelector("p[class=info]");
            if (message != null) {
              window.close();
            }
        }
    }

     if (inHref("page=nukes?target=")) {
        let radiation = numberFromIndicator('.nukestat-radiation')
        radiation = radiation.substring(0, radiation.length - 1);
        const targeted = numberFromIndicator('.nukestat-targeted')
        const incoming = numberFromIndicator('.nukestat-incoming')
        const total = parseInt(targeted) + 4*parseInt(radiation) + parseInt(incoming)
        if(total>=400){
             var bar = document.querySelector('.nukeiconbar')
        var div = document.createElement('div')
        var message = document.createElement('p')
        message.innerText = "This nation already has 400 or more targets + radiation. Consider aiming at someone else"
        div.style.backgroundColor= "red"
        message.style.fontSize = "30px"
        message.style.margin = "5px"
        div.style.padding = "5px"
        div.appendChild(message)
        var fineprint = document.createElement('i')
        fineprint.innerText = "This message has been inserted by JEFFnet"
        div.appendChild(fineprint)
        bar.insertAdjacentElement('afterend', div)
        }
       
    }

    Mousetrap.bind(
        ['w'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus].click();
                moveFocus()
            }
        }
    )

    Mousetrap.bind(
        ['e'],
        function (ev) {
            if (onSheet()) {
                moveFocus()
            }
        }
    )

    Mousetrap.bind(
        ['1'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus].style.color = "black";
                focus = 0;
                document.querySelectorAll('a')[focus].style.color = "red";
                document.querySelectorAll('a')[focus].scrollIntoView();
            }
        }
    )

    Mousetrap.bind(
        ['2'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus].style.color = "black";
                focus = links*(stratl);
                document.querySelectorAll('a')[focus].style.color = "red";
                document.querySelectorAll('a')[focus].scrollIntoView();
            }
        }
    )

    Mousetrap.bind(
        ['3'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus].style.color = "black";
                focus = links*(stratl+econl);
                document.querySelectorAll('a')[focus].style.color = "red";
                document.querySelectorAll('a')[focus].scrollIntoView();
            }
        }
    )

    Mousetrap.bind(
        ['4'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus].style.color = "black";
                focus = links*(stratl+econl+intell);
                document.querySelectorAll('a')[focus].style.color = "red";
                document.querySelectorAll('a')[focus].scrollIntoView();
            }
        }
    )

    Mousetrap.bind(
        ['q'],
        function (ev) {
            document.querySelectorAll('a')[focus].style.color = "black";
            if (focus >= links) {
                focus -= links;
            }
            document.querySelectorAll('a')[focus].style.color = "red";
            if (focus > 1) { document.querySelectorAll('a')[focus - 1].scrollIntoView(); }
        }
    )

    Mousetrap.bind(['b'], 
        function(ev) {
            window.history.back();
        }
    )

    Mousetrap.bind(['x'],
        function(ev) {
            window.close();
        }
    )

    Mousetrap.bind(['r'],
        function (ev) {
            window.location.reload();
        })

    Mousetrap.bind(['g'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus + 1].click();
                moveFocus()
            }

            else if (onProductionPage()) {
                if ($('span.fancylike').text().indexOf("Military") > -1) {
                    $('.button[name="convertproduction"][value^="nukes"]').first().trigger('click');
                }
                else if ($('span.fancylike').text().indexOf("Strategic") > -1) {
                    $('.button[name="convertproduction"][value^="shield"]').first().trigger('click');
                }
                else if ($('span.fancylike').text().indexOf("Economic") > -1) {
                    $('.button[name="convertproduction"][value^="shield"]').first().trigger('click');
                }
                else if ($('span.fancylike').text().indexOf("Intel") > -1) {
                    $('.button[name="convertproduction"][value^="shield"]').first().trigger('click');
                }

                }

            else {
                window.location.href = "https://www.nationstates.net/page=nukes/view=production"
            }
        }
    )

    Mousetrap.bind(['a'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus + 1].click();
                moveFocus()
            }

            else if (onProductionPage()) {
                document.querySelector('.button[name="convertproduction"][value^="nukes"]').click();
            }

            else {
                window.location.href = "https://www.nationstates.net/page=nukes/view=production";
            }
        })
    Mousetrap.bind(['d'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus + 1].click();
                moveFocus()
            }
            else if (onProductionPage()) {
                document.querySelector('.button[name="convertproduction"][value^="shield"]').click();
            }
            else {
                window.location.href = "https://www.nationstates.net/page=nukes/view=production";
            }
        })

    Mousetrap.bind(['s'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus + 2].click();
                moveFocus()
            }
            else if (inHref("/view=incoming")) {
                const buttons = document.querySelectorAll('.button[name="defend"]')
                if (buttons.length > 0) {
                    buttons[Math.floor(Math.random() * buttons.length)].click();
                } else {
                    window.location.reload();
                }
            } else {
                window.location.href = "https://www.nationstates.net/page=faction/fid=" + faction + "/view=incoming";
            }
        })

    Mousetrap.bind(['t'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus + 3].click();
                moveFocus()
            }
            else if (inHref("page=faction") && !inHref("view=nations")) {
                document.querySelector('a.nukestat-nations').click();
            } else if (inHref("page=faction") && inHref("view=nations")) {

                let livingNationsNodeList = document.querySelectorAll("ol > li > a.nlink");
                let livingNations = Array.from(livingNationsNodeList).filter((nation) => { return nation.nextSibling.className != 'nukedestroyedicon' });

                if (livingNations.length > 0) {
                    const tName = livingNations[Math.floor(Math.random() * livingNations.length)].href
                    tName.replace('/nation=', '');
                    tName.replace('/page=nukes', '');
                    window.location.href = "https://www.nationstates.net/nation=" + tName + "/page=nukes?target=tName";
                } else {
                    window.location.reload();
                }
            } else if (inHref("page=nukes?target=")) {

                const buttons = document.querySelectorAll('.button[name="nukes"]');

                if (buttons.length > 0) {
                    buttons[0].click();
                }
            } else {
                window.location.href = "https://www.nationstates.net/page=faction/fid=" +target + "/view=nations/start=" + Math.floor(Math.random() * targetNum);
            }
        })
    Mousetrap.bind(['f'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus + 4].click();
                moveFocus()
            }
            else if (inHref("page=nukes/view=targets") && inHref("nation=" + nname())) {
                const ready = document.querySelector('.button[name="launch"]')
                if (ready != null) {
                    ready.click()
                } else if (!turbo) {
                    window.location.reload();
                }
            } else {
                window.location.href = "https://www.nationstates.net/page=nukes/view=targets";
            }
        })

    Mousetrap.bind(['j'],
        function (ev) {
            if (onSheet()) {
                document.querySelectorAll('a')[focus + 5].click();
                moveFocus()
            }
            else { window.location.href = "https://www.nationstates.net/page=faction/fid=" + faction + "?consider_join_faction=1&join_faction=1"; }
        })

})();