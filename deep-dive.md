# Deep Dive

The LED control shows up as a serial device in Linux as well as Windows.
When interacting with a serial device, commonly following properties need to be known:

- baud rate
- character size
- stop bits
- etc.

(**notice:** see, e.g. the **serialport** crate documentation [1] for further )

In this case, two approaches have been taken to retrieve the user options.

## Intercepting Data

The reddit user **fairedemmy** [2] sniffed the data sent to the serial port.
By changing around values and looking at the intercepted data, it can be determined how the LEDs are set.

## Reverse Engineering the Data

**fairedemmy** [2] shared a link to `LedControl.exe` that is responsible for controlling the LEDs.

Once downloaded, the can be decompiled using tools, such as ILSpy [3] and dnSpy [4].
The decompilers are able to decompile the .NET bytecode into C#.
Looking over the code, the file `Form2` is of great interest.
In this file, one can find the code that configures the connection to the serial port:

```cs
    ...
		private void openPort()
		{
			this.sp.PortName = this.portName;
			this.sp.BaudRate = 10000;
			this.sp.DataBits = 8;
			this.sp.StopBits = StopBits.One;
			this.sp.Parity = Parity.None;
			this.sp.ReadTimeout = 200;
			try
			{
				this.sp.Open();
				this.openState = true;
			}
			catch (IOException)
			{
				this.openState = false;
			}
		}
    ...
```

Further down, e.g. the LED's mode is set to `off`:

```cs
    ...
		private void pictureBox1_guandeng_Click(object sender, EventArgs e)
		{
			this.CurrentLedMode = 4;
			this.arrSerialData[1] = (byte)this.CurrentLedMode;
			this.Send_data(this.arrSerialData);
			this.setPicboxImage();
		}
    ...
```

Looking over the code, the existing findings, which were obtained by intercepting the data sent to the serial port, were confirmed.

Sadly, no further modes were found, such as a solid colour mode. 

[1] https://docs.rs/serialport/latest/serialport/trait.SerialPort.html  
[2] https://old.reddit.com/user/fairedemmy  
[3] https://github.com/icsharpcode/ILSpy  
[4] https://github.com/dnSpyEx/dnSpy  