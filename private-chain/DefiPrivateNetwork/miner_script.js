var seconds = 10;

function checkWork() {
    if (eth.getBlock("pending").transactions.length > 0) {
        if (eth.mining){
             admin.sleep(seconds);
	     checkWork();
	     return;
	}
        console.log("== Pending transactions! Mining...");
	personal.unlockAccount(eth.accounts[0],"123");
        miner.start();
	checkWork();
    } else {
        miner.stop();  // This param means nothing
        console.log("== No transactions! Mining stopped.");
    }
    admin.sleep(seconds);
//    await setTimeout(10000);
//    console.log("Waited 10s");
    checkWork();
}

//eth.filter("latest", function(err, block) { checkWork(); });
//eth.filter("pending", function(err, block) { checkWork(); });

checkWork();
