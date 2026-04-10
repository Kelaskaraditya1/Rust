package com.starkindustries.java_backend.exception;

import org.springframework.http.HttpStatus;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.NoArgsConstructor;

@AllArgsConstructor
@NoArgsConstructor
@Builder
public class CustomException extends RuntimeException {

    public HttpStatus statusCode;
    public String message;
    public long timeStamp;

    public CustomException(HttpStatus statusCode,String message){
        super(message);
        this.statusCode=statusCode;
        this.message=message;
        this.timeStamp=System.currentTimeMillis();
    }
    
}
