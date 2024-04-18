var ip = fetch("https://ip.shronk.nicroxio.co.uk").then((r) =>
  r.json().then((data) => {
    console.debug("Data: %s", data);
    const dataDisplay = document.getElementById("dataDisplay");

    const ipElement = document.createElement("p");
    const cityElement = document.createElement("p");
    const countryElement = document.createElement("p");
    const brexitRequired = document.createElement("p");
    const isoCode = document.createElement("p");

    ipElement.textContent = "IP Address: " + data.ip;
    cityElement.textContent = "City Name: " + data.city_name;
    countryElement.textContent = "Country Name: " + data.country_name;
    brexitRequired.textContent =
      "Brexit Required: " + data.is_in_european_union;
    isoCode.textContent = "Iso Code: " + data.iso_code;

    dataDisplay.appendChild(ipElement);
    dataDisplay.appendChild(cityElement);
    dataDisplay.appendChild(countryElement);
    dataDisplay.appendChild(brexitRequired);
    dataDisplay.appendChild(isoCode);
  })
);
