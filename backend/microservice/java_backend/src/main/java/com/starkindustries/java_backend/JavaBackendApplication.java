package com.starkindustries.java_backend;

import java.util.HashMap;
import java.util.Map;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.starkindustries.java_backend.service.UsersService;

@SpringBootApplication
@RestController
@RequestMapping("/api/v1")
public class JavaBackendApplication {

	@Autowired
	public UsersService usersService;

	@GetMapping("/greetings")
	public ResponseEntity<?> greetings(){

		Map<String,String> response = new HashMap<>();
		response.put("health","healthy");
		response.put("status","Ok");
		response.put("message","Backend running on port 8080");

		return ResponseEntity.status(HttpStatus.OK).body(response);

	}

	@GetMapping("/users")
	public ResponseEntity<?> getUsers(){

		return ResponseEntity.status(HttpStatus.OK).body(this.usersService.getDummyUsers());
	}

	public static void main(String[] args) {
		SpringApplication.run(JavaBackendApplication.class, args);
	}

}
