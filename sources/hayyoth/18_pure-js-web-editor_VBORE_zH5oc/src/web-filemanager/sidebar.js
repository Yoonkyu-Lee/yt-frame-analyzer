// Reconstructed from frames (87:50-87:53). The final body of initFiles was not
// visible; the last frame shows `const file = localforage.get` unfinished.
// Lines marked RECONSTRUCTED follow the localForage getItem docs he opened at 86:52.
function initFiles(tabs, detailCtn) {
    localforage.keys().then(function (keys) {
        // An array of all the key names.
        console.log(keys);
        keys.forEach(key => {
            console.log(key)
            localforage.getItem(key).then(function (file) {   // RECONSTRUCTED
                appendFile(file, tabs, detailCtn);
            });                                               // RECONSTRUCTED
        });
    })
}

function appendFile(file, tabs, detailCtn) {
    const urlFile = URL.createObjectURL(file);

    const tab = document.createElement("div");
    tab.innerHTML = file.name;
    tab.className = "sidebar-tab";

    tab.addEventListener("click", function () {
        console.log(file);
        detailCtn.innerHTML = "";

        if (file.type.startsWith("image/")) {
            const img = document.createElement("img");
            img.src = urlFile;
            detailCtn.append(img);
        }
        else if (file.type.startsWith("video/")) {
            const video = document.createElement("video");
            video.controls = true;
            const source = document.createElement("source");
            source.src = urlFile;
            video.append(source);

            detailCtn.append(video);
        }

    })

    tabs.append(tab);
}

export function sidebar(detailCtn) {
    const ctn = document.createElement("div");
    ctn.className = "sidebar"

    const sidebarNav = document.createElement("div");
    sidebarNav.className = "sidebar-nav";

    const inpUpload = document.createElement("input");

    const btnUpload = document.createElement("button");
    btnUpload.className = "btn-upload";

    const tabs = document.createElement("div");
    tabs.className = "sidebar-tabs";

    inpUpload.type = "file";
    inpUpload.multiple = true;
    // can it storage these files forever
    inpUpload.addEventListener("change", () => {
        for (const file of inpUpload.files) {
            localforage.setItem(file.name, file);
            appendFile(file, tabs, detailCtn);
        }
    });


    btnUpload.innerHTML = "Upload";
    btnUpload.addEventListener("click", function () {
        inpUpload.click();
    })

    sidebarNav.append("HFiles")
    sidebarNav.append(btnUpload);

    initFiles(tabs, detailCtn);

    ctn.append(sidebarNav, tabs);
    return ctn;
}
