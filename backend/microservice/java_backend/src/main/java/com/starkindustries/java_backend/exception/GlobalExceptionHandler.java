package com.starkindustries.java_backend.exception;

import java.util.HashMap;
import java.util.Map;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import org.springframework.web.servlet.mvc.method.annotation.ResponseEntityExceptionHandler;

import lombok.extern.slf4j.Slf4j;


@RestControllerAdvice
@Slf4j
public class GlobalExceptionHandler extends ResponseEntityExceptionHandler{

    @ExceptionHandler(CustomException.class)
    public ResponseEntity<?> globalExceptionHandler(CustomException customException){
        
        log.info("Error status: {}",customException.statusCode);
        log.info("Error message: {}",customException.message);

        Map<String,Object> response = new HashMap<>();

        response.put("statusCode",customException.statusCode);
        response.put("message",customException.message);
        response.put("timeStamp",customException.timeStamp);

        return ResponseEntity.status(HttpStatus.OK).body(response);

    }
    
}
