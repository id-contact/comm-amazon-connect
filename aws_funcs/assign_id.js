// Set CALL_URL to something like http://proxy.irma.bellen.tweede.golf/link_phone

const http = require('http'); //modify this to https for production

exports.handler = (data, _context, callback) => {
    const req = http.request(process.env.CALL_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
    }, (res) => {
        res.on('data', () => {}); // eat data (we dont need it but)
        res.on('end', () => {
            if (res.statusCode < 200 || res.statusCode > 300) {
                callback(Error(`Backed ${res.statusCode}`));
            } else {
                callback(null, {});
            }
        });
    });
    req.on('error', callback);
    req.write(JSON.stringify({
        dtmf: data.Details.ContactData.Attributes.dtmf,
        sessionid: data.Details.ContactData.ContactId
    }));
    req.end();
}
