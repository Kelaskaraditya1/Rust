package com.starkindustries.java_backend;

import java.util.HashMap;
import java.util.Map;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.starkindustries.java_backend.service.EmailService;
import com.starkindustries.java_backend.service.UsersService;

@SpringBootApplication
@RestController
@RequestMapping("/api/v1")
public class JavaBackendApplication {

	@Autowired
	public UsersService usersService;

	@Autowired
	public EmailService  emailService;

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

	@PostMapping("/send/email/{email}/{name}")
	public ResponseEntity<?> sendEmail(@PathVariable("email") String email, @PathVariable("name") String name){
		
		Map<String,Object> response = new HashMap<>();
		response.put("timeStamp",System.currentTimeMillis());

		if(email==null || email.isBlank() || email.isEmpty()){
			response.put("statusCode",HttpStatus.BAD_REQUEST);
			response.put("message", "enter proper email");

			return ResponseEntity.status(HttpStatus.BAD_REQUEST).body(response);
		}

		if(name==null || name.isBlank() || name.isEmpty()){
			response.put("statusCode",HttpStatus.BAD_REQUEST);
			response.put("message", "enter proper name");

			return ResponseEntity.status(HttpStatus.BAD_REQUEST).body(response);
		}

		if(this.emailService.sendEmail(email, name)){
			
			response.put("satusCode",HttpStatus.OK);
			response.put("message", "Email send to "+email+" sucessfully");

			return ResponseEntity.status(HttpStatus.OK).body(response);
		}else{
			
			response.put("satusCode",HttpStatus.INTERNAL_SERVER_ERROR);
			response.put("message", "failed to send email");

			return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(response);
		}

	}

	public static void main(String[] args) {
		SpringApplication.run(JavaBackendApplication.class, args);
	}

}
