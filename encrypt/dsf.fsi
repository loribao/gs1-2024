open System.Diagnostics;

let print_run = new DataReceivedEventHandler(fun (sender: obj) (e: DataReceivedEventArgs) -> Console.WriteLine(e.Data))

let run (cmd) =
    let startInfo = ProcessStartInfo()
    startInfo.FileName <- "/bin/zsh"
    startInfo.Arguments <- $"-c \"{cmd}\""

    startInfo.RedirectStandardOutput <- true
    startInfo.UseShellExecute <- false
    startInfo.CreateNoWindow <- false

    use process = new Process()
    process.EnableRaisingEvents <- true
    process.StartInfo <- startInfo
    process.OutputDataReceived.AddHandler(print_run)
    process.Start()
    process.BeginOutputReadLine()
    process.WaitForExit()
    process.Close()

let command = "tshark -r decrypt.pcap -Y \"dns\" -T fields -e ip.dst > gp_ips.txt"
run command
