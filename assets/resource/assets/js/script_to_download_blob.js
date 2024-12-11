function downloadBlob(blobUrl) {
  const xhr = new XMLHttpRequest();
  xhr.open('GET', blobUrl, true);
  xhr.responseType = 'blob'
  xhr.onload = function () {
    if (xhr.status === 200) {
      const blob = xhr.response;
      const reader = new FileReader()
      reader.onload = function () {
        let base64data = reader.result;
        let base64ContentArray = base64data.split(",");
        let decodedFile = base64ContentArray[1];

        window.flutter_inappwebview.callHandler('Download', decodedFile);
      };

      reader.onerror = function () {
        console.log('Error reading blob data.');
      };

      reader.readAsDataURL(blob);
    } else {
      console.log("HTTP Error: ${xhr.status}");
    }
  };

  xhr.onerror = function () {
    console.log('Failed to download blob.');
  };

  xhr.send();
}