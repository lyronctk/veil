const express = require("express");
var cors = require('cors')
const app = express();
app.use(express.json())
app.use(cors())

const PORT = 8000;

app.post("/upload_signed_tx", (req, res) => {
    console.log("upload approvals hit");
    console.log(req.body.approvals);
    res.send(JSON.stringify({success: true}));
});
  
app.listen(PORT, console.log(`Server started on port ${PORT}`));